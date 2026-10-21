// `nub run capture` in site/: the module Reylo's content mapper returns for every `.sql` block a
// `ts twoslash` block of the docs imports, written to lib/sql-modules.json, which
// lib/remark-mapper.ts hides above each block's `// ---cut---` at build time.
//
//   nub run capture                          every page under content/docs
//   nub run capture -- content/drafts/x.mdx  those pages too
//   nub run capture -- --schema mysql=f.sql  a dialect's schema, for a page that shows none
//
// For each page and dialect it builds the page's schema (the `schema.sql` blocks, one per
// dialect, in the accordion near the top) in a scratch database: Postgres and MySQL
// `reylo_capture` (REYLO_CAPTURE_POSTGRES and REYLO_CAPTURE_MYSQL name others; the database is
// dropped and made again, so its name must start with reylo_capture), SQLite a temporary file. It
// compiles the syntax the engine does not build yet (typed JSON parameters, `::` on MySQL and
// SQLite), has the loader's own Session analyze each statement through the dialect's driver, and
// renders the module exactly as src/mapper.ts does, with each typed JSON parameter's type computed
// from its schema (scripts/schema-type.ts) where the loader still says `unknown`. An error in a
// statement a block imports fails the run; one in a statement nothing imports is a warning.
import { mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, symlinkSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { DatabaseSync } from 'node:sqlite';
import { createProcessor } from '@mdx-js/mdx';
import { remarkCodeTab } from 'fumadocs-core/mdx-plugins';
import postgres from 'postgres';
import mysql from 'mysql2/promise';
import { Session } from '../../src/analyze.ts';
import { ident, renderParamType, renderType } from '../../src/render.ts';
import type { Analysis } from '../../src/engine.ts';
import { collect, plan, isStatement, DIALECT_NAME, SCHEMA_FILE, type Block, type Dialect, type MdNode, type Modules } from '../lib/remark-mapper.ts';
import { schemaType } from './schema-type.ts';

const site = path.resolve(import.meta.dirname, '..');
const URLS: Record<Exclude<Dialect, 'sqlite'>, string> = {
  postgres: process.env.REYLO_CAPTURE_POSTGRES ?? 'postgres://postgres@localhost:54329/reylo_capture',
  mysql: process.env.REYLO_CAPTURE_MYSQL ?? 'mysql://root@127.0.0.1:54336/reylo_capture',
};
// The driver each scratch project names, whose defaults the types follow, as for a project with
// that driver: postgres.js, mysql2, and node:sqlite (no package).
const DRIVER: Record<Dialect, string | undefined> = { postgres: 'postgres', mysql: 'mysql2', sqlite: undefined };
const ORDER: Dialect[] = ['postgres', 'mysql', 'sqlite'];

// ---- Arguments ----

const files: string[] = [];
const schemaFiles: Partial<Record<Dialect, string>> = {};
const args = process.argv.slice(2);
for (let i = 0; i < args.length; i++) {
  const arg = args[i]!;
  if (arg === '--') continue;
  const schema = /^--schema(?:=(.*))?$/.exec(arg);
  if (schema) {
    const value = schema[1] ?? args[++i];
    const m = /^(postgres|mysql|sqlite)=(.+)$/.exec(value ?? '');
    if (!m) fail(`--schema takes <dialect>=<file>, as in --schema mysql=schema.sql; got ${value ?? 'nothing'}`);
    schemaFiles[m[1] as Dialect] = readFileSync(path.resolve(m[2]!), 'utf8');
  } else if (arg.startsWith('-')) fail(`unknown option ${arg}`);
  else files.push(path.resolve(arg));
}
const docs = path.join(site, 'content', 'docs');
const pages = [...new Set([...mdxFiles(docs), ...files])];

function mdxFiles(dir: string): string[] {
  return readdirSync(dir, { withFileTypes: true }).flatMap((e) => (e.isDirectory() ? mdxFiles(path.join(dir, e.name)) : e.name.endsWith('.mdx') ? [path.join(dir, e.name)] : []));
}

function fail(message: string): never {
  console.error(`capture: ${message}`);
  process.exit(1);
}

// ---- The page, as the build reads it ----

/** The page's tree as Fumadocs has it when lib/remark-mapper.ts runs: MDX parsed, then code tabs grouped. */
function parse(file: string): MdNode {
  // fumadocs-mdx takes the frontmatter off before it parses; blank lines keep the line numbers.
  const text = readFileSync(file, 'utf8').replace(/^---\n[\s\S]*?\n---\n/, (m) => '\n'.repeat(m.split('\n').length - 1));
  const processor = createProcessor();
  const tree = processor.parse(text) as unknown as MdNode;
  (remarkCodeTab as unknown as (this: unknown) => (tree: MdNode) => void).call(processor)(tree);
  return tree;
}

// ---- The syntax the engine does not build yet ----

const NAME = String.raw`:[A-Za-z_]\w*`;
const SCHEMA_STRING = String.raw`'((?:[^']|'')*)'`;
/** A cast's target type on MySQL and SQLite: `int`, `unsigned`, `char(10)`, `decimal(10, 2)`. */
const TARGET = String.raw`[A-Za-z_]\w*(?:\s*\(\s*\d+(?:\s*,\s*\d+)?\s*\))?`;

/** Each typed JSON parameter of the statement as written: its name and its schema. */
function jsonParams(sql: string): { name: string; schema: unknown }[] {
  return [...sql.matchAll(new RegExp(String.raw`\(?(${NAME})\)?::jsonb?\(${SCHEMA_STRING}\)`, 'g'))].map((m) => {
    const text = m[2]!.replace(/''/g, "'");
    try {
      return { name: m[1]!.slice(1), schema: JSON.parse(text) };
    } catch (e) {
      throw new Error(`the schema of ${m[1]} is not JSON: ${(e as Error).message}`);
    }
  });
}

/**
 * The statement the engine reads: Postgres `:x::jsonb('…')` → `:x::jsonb`; MySQL `(:x)::json('…')`
 * → `cast(:x as json)` and `(:x)::type` → `cast(:x as type)`; SQLite `:x::json('…')` → `:x` and
 * `(:x)::type` → `cast(:x as type)` (each with or without the parentheses).
 */
function compile(sql: string, dialect: Dialect): string {
  if (dialect === 'postgres') return sql.replace(new RegExp(String.raw`::(jsonb?)\(${SCHEMA_STRING}\)`, 'g'), '::$1');
  const json = new RegExp(String.raw`\(?(${NAME})\)?::json\(${SCHEMA_STRING}\)`, 'g');
  const cast = new RegExp(String.raw`\((${NAME})\)::(${TARGET})|(${NAME})::(${TARGET})(?![\w(])`, 'g');
  return sql
    .replace(json, dialect === 'mysql' ? 'cast($1 as json)' : '$1')
    .replace(cast, (_, a: string | undefined, at: string | undefined, b: string, bt: string) => `cast(${a ?? b} as ${at ?? bt})`);
}

// ---- The module, as src/mapper.ts renders it ----

/**
 * The module src/mapper.ts's `transform` returns for the statement, copied from there (keep the two
 * in step): `written` is the file's text, declared as `raw`, and `sql` what the engine reads; the
 * module's `sql` is the engine's compiled text, the statement as Reylo sends it to the database.
 * One change: a typed JSON parameter takes the type computed from its schema (`json`), where the
 * loader says `unknown`.
 */
async function render(session: Session, written: string, sql: string, json: Map<string, string>): Promise<{ text: string; analysis: Analysis } | { error: string }> {
  const project = session.project;
  const paramType = (q: Analysis['params'][number], rendered: string): string => {
    if (json.has(q.name)) return json.get(q.name)!;
    if (q.type.kind === 'unknown') return 'unknown';
    return q.nullable ? `${rendered} | null` : rendered;
  };
  const emit = (compiled: string, paramsType: string, rowType: string, cardinality: string, names: string[], valuesType = 'unknown[]') =>
    [
      `import type { Query } from "reylo";`,
      `export type Params = ${paramsType};`,
      `export type Values = ${valuesType};`,
      `export type Row = ${rowType};`,
      `export declare const sql: ${JSON.stringify(compiled)};`,
      `export declare const raw: ${JSON.stringify(written)};`,
      `export declare const params: readonly [${names.map((n) => JSON.stringify(n)).join(', ')}];`,
      `declare const query: Query<${paramsType}, ${rowType}, ${JSON.stringify(cardinality)}, ${JSON.stringify(project.dialect)}, ${valuesType}> & { readonly sql: typeof sql; readonly raw: typeof raw };`,
      `export default query;`,
      ``,
    ].join('\n');
  let names: string[] = [];
  try {
    ({ params: names } = session.lex(sql));
  } catch {}
  const outcome = await session.analyze(sql);
  if (!outcome.ok) return { error: outcome.message };
  const { analysis } = outcome;
  const dialect = project.dialect;
  if (dialect === 'mysql') names = analysis.params.map((q) => q.name);
  // The mapper places these in the file as errors; a statement on the page must have none.
  if (analysis.diagnostics.length) return { error: analysis.diagnostics.map((d) => d.message).join('; ') };
  if (project.strict) {
    const untyped = analysis.params.filter((q) => q.type.kind === 'unknown' && !json.has(q.name));
    if (untyped.length) return { error: `strict: nothing types ${untyped.map((q) => `:${q.name}`).join(', ')}` };
  }
  const inferred = analysis.cardinality;
  const order = names;
  if (dialect === 'postgres' && analysis.kind === '') return { error: 'the statement is empty' };
  const columns = analysis.columns.map((c) => `${ident(c.name)}: ${renderType(c.type, false, dialect)}${c.nullable ? ' | null' : ''}`);
  const typed = analysis.params.map((q) => [q.name, paramType(q, renderParamType(q.type, dialect))] as const);
  const paramTypes = typed.map(([name, type]) => `${ident(name)}: ${type}`);
  const types = new Map(typed);
  const labels = order.map((n) => (/^\d+$/.test(n) ? `$${n}` : n));
  const labeled = labels.every((l) => /^[A-Za-z_$][\w$]*$/.test(l));
  const valuesType = `[${order.map((n, i) => `${labeled ? `${labels[i]}: ` : ''}${types.get(n) ?? 'unknown'}`).join(', ')}]`;
  return { text: emit(analysis.rewritten.sql, paramTypes.length ? `{ ${paramTypes.join('; ')} }` : 'Record<never, never>', columns.length ? `{ ${columns.join('; ')} }` : 'never', inferred, order, valuesType), analysis };
}

// ---- The scratch databases ----

/** A Postgres or MySQL URL's database, which the run drops and makes again: only a scratch one. */
function scratch(url: string): { db: string; admin: URL } {
  const u = new URL(url);
  const db = decodeURIComponent(u.pathname.slice(1));
  if (!/^reylo_capture\w*$/.test(db)) fail(`${url} names the database "${db}"; the capture drops and makes its database again, so its name must start with reylo_capture`);
  return { db, admin: u };
}

async function buildPostgres(schema: string): Promise<string> {
  const { db, admin } = scratch(URLS.postgres);
  admin.pathname = '/postgres';
  const server = postgres(admin.href, { max: 1, onnotice: () => {} });
  try {
    await server.unsafe(`drop database if exists "${db}" with (force)`);
    await server.unsafe(`create database "${db}"`);
  } finally {
    await server.end();
  }
  const sql = postgres(URLS.postgres, { max: 1, onnotice: () => {} });
  try {
    await sql.unsafe(schema).simple();
  } finally {
    await sql.end();
  }
  return URLS.postgres;
}

async function buildMysql(schema: string): Promise<string> {
  const { db, admin } = scratch(URLS.mysql);
  admin.pathname = '/';
  const conn = await mysql.createConnection({ uri: admin.href, multipleStatements: true });
  try {
    await conn.query(`drop database if exists \`${db}\``);
    await conn.query(`create database \`${db}\``);
    await conn.query(`use \`${db}\``);
    await conn.query(schema);
  } finally {
    await conn.end();
  }
  return URLS.mysql;
}

function buildSqlite(dir: string, schema: string): string {
  const db = new DatabaseSync(path.join(dir, 'capture.db'));
  try {
    db.exec(schema);
  } finally {
    db.close();
  }
  return 'file:./capture.db';
}

/**
 * A project of its own per dialect, as an app using Reylo has: its `package.json` names the driver
 * (which picks the types' defaults), and the driver resolves from the site's node_modules.
 */
function project(root: string, dialect: Dialect): string {
  const dir = path.join(root, dialect);
  rmSync(dir, { recursive: true, force: true });
  mkdirSync(dir, { recursive: true });
  const driver = DRIVER[dialect];
  writeFileSync(path.join(dir, 'package.json'), JSON.stringify({ name: `reylo-capture-${dialect}`, private: true, type: 'module', dependencies: driver ? { [driver]: '*' } : {} }, null, 2));
  symlinkSync(path.join(site, 'node_modules'), path.join(dir, 'node_modules'), 'dir');
  return dir;
}

// ---- The run ----

const modules: Modules = {};
const errors: string[] = [];
const warnings: string[] = [];
const root = mkdtempSync(path.join(tmpdir(), 'reylo-capture-'));
let captured = 0;
let analyzed = 0;

try {
  for (const page of pages) {
    const shown = path.relative(site, page).startsWith('..') ? page : path.relative(site, page);
    let blocks: Block[];
    let needed: Set<Block>;
    try {
      blocks = collect(parse(page));
      needed = new Set(plan(blocks).flatMap((p) => p.variants.flatMap((v) => v.needs.map((n) => n.block))));
    } catch (e) {
      errors.push(`${shown}: ${(e as Error).message}`);
      continue;
    }
    const statements = blocks.filter(isStatement);
    if (!statements.length) continue;
    const schemas: Partial<Record<Dialect, string>> = { ...schemaFiles };
    for (const d of ORDER) {
      const own = blocks.filter((b) => b.name === SCHEMA_FILE && b.dialect === d);
      if (own.length) schemas[d] = own.map((b) => b.node.value).join('\n\n');
    }
    for (const dialect of ORDER) {
      const mine = statements.filter((b) => b.dialect === dialect);
      if (!mine.length) continue;
      const report = (block: Block, message: string) => (needed.has(block) ? errors : warnings).push(`${shown}:${block.line}: ${DIALECT_NAME[dialect]} ${block.name}: ${message}\n${indent(block.node.value)}`);
      const schema = schemas[dialect];
      if (schema === undefined) {
        for (const block of mine) report(block, `the page has no ${DIALECT_NAME[dialect]} ${SCHEMA_FILE} block to build its schema from; add one, or pass --schema ${dialect}=<file>`);
        continue;
      }
      const dir = project(root, dialect);
      let url: string;
      try {
        url = dialect === 'postgres' ? await buildPostgres(schema) : dialect === 'mysql' ? await buildMysql(schema) : buildSqlite(dir, schema);
      } catch (e) {
        errors.push(`${shown}: the ${DIALECT_NAME[dialect]} ${SCHEMA_FILE} fails: ${(e as Error).message}`);
        continue;
      }
      const session = await Session.open(dir, { DATABASE_URL: url });
      try {
        const done = new Set<string>();
        for (const block of mine) {
          const key = `${block.name}\0${block.hash}`;
          if (done.has(key) && !needed.has(block)) continue;
          done.add(key);
          const written = block.node.value;
          let sql: string;
          const json = new Map<string, string>();
          try {
            sql = compile(written, dialect);
            for (const p of jsonParams(written)) json.set(p.name, schemaType(p.schema as never, dialect, `:${p.name}`));
          } catch (e) {
            report(block, (e as Error).message);
            continue;
          }
          const result = await render(session, `${written}\n`, `${sql}\n`, json);
          if ('error' in result) {
            report(block, result.error);
            continue;
          }
          analyzed++;
          if (!needed.has(block)) {
            console.log(`${DIALECT_NAME[dialect].padEnd(8)} ${block.name}: ${summary(result.text)}  (nothing imports it)`);
            continue;
          }
          const slot = ((modules[dialect] ??= {})[block.name!] ??= {});
          if (slot[block.hash] === result.text) continue;
          if (slot[block.hash] !== undefined) {
            errors.push(`${shown}:${block.line}: ${DIALECT_NAME[dialect]} ${block.name}: another page has the same statement with other types (another schema); give one of them another file name`);
            continue;
          }
          slot[block.hash] = result.text;
          captured++;
          console.log(`${DIALECT_NAME[dialect].padEnd(8)} ${block.name}: ${summary(result.text)}`);
        }
      } finally {
        await session.close();
      }
    }
  }
} finally {
  rmSync(root, { recursive: true, force: true });
}

function indent(text: string) {
  return text.replace(/^/gm, '    ');
}

/** The module's parameters, row and row count on one line. */
function summary(module: string): string {
  const m = /declare const query: Query<([\s\S]*)> & \{ readonly sql/.exec(module);
  return m ? m[1]!.replace(/, "(postgres|mysql|sqlite)", \[.*\]$/, '') : module;
}

for (const w of warnings) console.warn(`warning: ${w}`);
if (errors.length) {
  for (const e of errors) console.error(`error: ${e}`);
  console.error(`capture: ${errors.length} error${errors.length === 1 ? '' : 's'}; lib/sql-modules.json is unchanged.`);
  process.exit(1);
}

// Sorted, so a capture that changes nothing changes nothing in git.
const sorted = Object.fromEntries(
  ORDER.filter((d) => modules[d]).map((d) => [d, Object.fromEntries(Object.entries(modules[d]!).sort(([a], [b]) => a.localeCompare(b)).map(([name, byHash]) => [name, Object.fromEntries(Object.entries(byHash).sort(([a], [b]) => a.localeCompare(b)))]))]),
);
const out = path.join(site, 'lib', 'sql-modules.json');
writeFileSync(out, JSON.stringify({ $comment: 'Written by `nub run capture` (scripts/capture-types.ts); read by lib/remark-mapper.ts. Never edit by hand.', ...sorted }, null, 2) + '\n');
console.log(`capture: ${analyzed} statement${analyzed === 1 ? '' : 's'} analyzed on ${pages.length} page${pages.length === 1 ? '' : 's'}; ${captured} module${captured === 1 ? '' : 's'} imported → ${path.relative(site, out)}`);
