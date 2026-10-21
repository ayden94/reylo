import { createHash } from 'node:crypto';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';

// Twoslash runs TypeScript's JavaScript API (6.0, from fumadocs-twoslash), which cannot run a
// content mapper, and Reylo's mapper needs a database. So `nub run capture` (scripts/
// capture-types.ts) runs the loader's own analysis on every `.sql` block of the pages, with the
// scratch databases, and writes the module the mapper returns for each into lib/sql-modules.json.
// This plugin reads that file: each `ts twoslash` block that imports `./<name>.sql` gets, above its
// `// ---cut---`, the module for the block it imports (`<name>.d.sql.ts`, which the import resolves
// to under allowArbitraryExtensions) and `db` declared as that dialect's client. A block written
// once under an example's SQL tabs becomes one copy per dialect, in <DialectTabs follow>. The
// build never needs a database, and fails when a block's module was not captured.
//
// The conventions (the contract between the page and this file):
// - A `sql` block's file name is its `title`, else its `file` meta attribute, which names the file
//   without showing it and which this plugin removes, else its `tab` when that is not a dialect
//   name; a block whose name ends in `.sql` is a statement, except `schema.sql`, the page's schema.
// - A `sql` block with `dialect="all"` is the same statement in every dialect: it is shown once,
//   analyzed in each, and a `ts twoslash` block that imports it follows the page's dialect choice,
//   as under a code-tab group of dialects.
// - A block's dialect is its `tab` when that is Postgres, MySQL or SQLite; else the enclosing
//   <Dialect value> (or <Tab value>) panel's; else its `dialect="postgres|mysql|sqlite"` meta
//   attribute's, which this plugin removes; else, for a `ts twoslash` block, the dialect of the
//   `.sql` blocks it imports; else Postgres.
// - An import resolves to the nearest block above with that file name and the importing block's
//   dialect, else to the first one below.
// - A `ts twoslash` block with no dialect of its own whose import resolves to a code-tab group of
//   dialects becomes <DialectTabs follow> with a <Dialect value> panel per dialect of that group.
// - `db` is declared unless the block declares or imports it; a block that names `Bun.` or imports
//   `bun` or `bun:sqlite` gets Bun's types, and one that names `D1Database` without importing it
//   gets Cloudflare's Workers types.
//
// scripts/capture-types.ts parses the pages as Fumadocs does and calls `plan` below, so the
// capture and the build always agree on which statement each import needs.

export type Attribute = { type: string; name?: string; value?: unknown };
export type MdNode = {
  type: string;
  name?: string | null;
  lang?: string | null;
  meta?: string | null;
  value?: string;
  attributes?: Attribute[];
  data?: { tab?: string; tabGroup?: string; [key: string]: unknown };
  children?: MdNode[];
  position?: { start: { line: number; offset?: number } };
};
type Code = MdNode & { type: 'code'; value: string };

export const DIALECTS = ['Postgres', 'MySQL', 'SQLite'] as const;
export type DialectName = (typeof DIALECTS)[number];
export type Dialect = 'postgres' | 'mysql' | 'sqlite';
const DIALECT_OF: Record<string, Dialect> = { postgres: 'postgres', mysql: 'mysql', sqlite: 'sqlite' };
export const DIALECT_NAME: Record<Dialect, DialectName> = { postgres: 'Postgres', mysql: 'MySQL', sqlite: 'SQLite' };
/** The client an example calls `db`: the one Basic usage builds in each dialect. */
const CLIENT: Record<Dialect, string> = {
  postgres: 'import("postgres").Sql',
  mysql: 'import("mysql2/promise").Pool',
  sqlite: 'import("better-sqlite3").Database',
};
/** A tab or panel value or a meta attribute naming a dialect, in any case. */
const dialectOf = (value: unknown): Dialect | undefined => (typeof value === 'string' ? DIALECT_OF[value.toLowerCase()] : undefined);

/** The file whose blocks hold each dialect's schema, in the accordion near the top of a page. */
export const SCHEMA_FILE = 'schema.sql';

export type Block = {
  node: Code;
  /** The node's parent, where an expansion replaces it. */
  parent: MdNode;
  /** Its position among every code block of the page. */
  order: number;
  name: string | undefined;
  dialect: Dialect;
  /** Where the dialect came from; `default` is Postgres for want of anything else. */
  from: 'tab' | 'panel' | 'meta' | 'default';
  /** The code-tab group (Fumadocs' CodeBlockTabs) the block is a tab of, when its tab is a dialect. */
  group: MdNode | undefined;
  line: number;
  hash: string;
};

/** The key a statement's module has in lib/sql-modules.json, under its dialect and file name. */
export const hashOf = (text: string) => createHash('sha256').update(text).digest('hex').slice(0, 16);

/** A Fumadocs meta attribute, such as `title="main.ts"`. */
const attribute = (meta: string | null | undefined, name: string) => new RegExp(`(?:^|\\s)${name}=(?:"([^"]*)"|'([^']*)'|(\\S+))`).exec(meta ?? '')?.slice(1).find((v) => v !== undefined);

export const isTwoslash = (node: MdNode) => (node.lang === 'ts' || node.lang === 'tsx') && /(?:^|\s)twoslash(?:\s|$)/.test(node.meta ?? '');

/**
 * Every code block of the page, in order, with its file name and dialect. Fumadocs' remarkCodeTab
 * has already run: a group of code tabs is a CodeBlockTabs element, and each block's `tab` and
 * `tab-group` have moved from its meta into `data`.
 */
export function collect(tree: MdNode): Block[] {
  const blocks: Block[] = [];
  const walk = (node: MdNode, ancestors: MdNode[]) => {
    for (const child of node.children ?? []) {
      if (child.type === 'code') {
        const code = child as Code;
        const tab = code.data?.tab;
        const tabDialect = dialectOf(tab);
        const panel = ancestors
          .concat(node)
          .findLast((a) => a.type === 'mdxJsxFlowElement' && (a.name === 'Dialect' || a.name === 'Tab') && dialectOf(attr(a, 'value')));
        const panelDialect = panel && dialectOf(attr(panel, 'value'));
        const metaDialect = attribute(code.meta, 'dialect');
        const every = metaDialect === 'all';
        if (metaDialect !== undefined && !every && !dialectOf(metaDialect)) throw new Error(`${where(code)}: dialect="${metaDialect}" names no dialect; write postgres, mysql, sqlite or all.`);
        const dialect = tabDialect ?? panelDialect ?? dialectOf(metaDialect) ?? 'postgres';
        const name = attribute(code.meta, 'title') ?? attribute(code.meta, 'file') ?? (tab && !tabDialect ? tab : undefined);
        const shared = { node: code, parent: node, name, line: code.position?.start.line ?? 0, hash: hashOf(code.value) };
        // The same statement in every dialect: one block per dialect, grouped as a code-tab group
        // of dialects is, so that an import of it expands into a panel per dialect.
        if (every) for (const each of DIALECTS) blocks.push({ ...shared, order: blocks.length, dialect: DIALECT_OF[each.toLowerCase()]!, from: 'meta', group: code });
        else
          blocks.push({
            ...shared,
            order: blocks.length,
            dialect,
            from: tabDialect ? 'tab' : panelDialect ? 'panel' : metaDialect ? 'meta' : 'default',
            group: tabDialect ? ancestors.concat(node).findLast((a) => a.type === 'mdxJsxFlowElement' && a.name === 'CodeBlockTabs') : undefined,
          });
      } else walk(child, ancestors.concat(node));
    }
  };
  walk(tree, []);
  return blocks;
}

function attr(node: MdNode, name: string): unknown {
  return node.attributes?.find((a) => a.type === 'mdxJsxAttribute' && a.name === name)?.value;
}
const where = (node: MdNode) => `line ${node.position?.start.line ?? '?'}`;

/** A `.sql` block that a page analyzes: every SQL block named `*.sql` but the schema (a TypeScript tab named Bun.sql is not one). */
export const isStatement = (block: Block) => (block.node.lang ?? 'sql') === 'sql' && !!block.name?.endsWith('.sql') && block.name !== SCHEMA_FILE;

/** `import getUser from "./get_user.sql"`. */
const IMPORT = /\b(?:from|import)\s+["']\.\/([^"']+\.sql)["']/g;

/** The file TypeScript looks up for `./name.sql` under allowArbitraryExtensions. */
export const declarationName = (file: string) => `${file.slice(0, -'.sql'.length)}.d.sql.ts`;

export type Need = { name: string; block: Block };
export type Variant = { dialect: Dialect; needs: Need[] };
export type Plan = { block: Block; variants: Variant[]; expand: boolean };

/**
 * What each `ts twoslash` block needs: per dialect it is shown in, the `.sql` block each of its
 * imports resolves to. A block that writes `// @filename: get_user.d.sql.ts` itself supplies that
 * module, and needs nothing for it.
 */
export function plan(blocks: Block[]): Plan[] {
  const statements = blocks.filter(isStatement);
  const resolve = (name: string, at: Block, dialect: Dialect | undefined) => {
    const matches = (b: Block) => b.name === name && (dialect === undefined || b.dialect === dialect);
    return statements.findLast((b) => b.order < at.order && matches(b)) ?? statements.find((b) => b.order > at.order && matches(b));
  };
  const plans: Plan[] = [];
  for (const block of blocks) {
    if (!isTwoslash(block.node)) continue;
    const code = block.node.value;
    const names = [...new Set([...code.matchAll(IMPORT)].map((m) => m[1]!))].filter((name) => !code.includes(`@filename: ${declarationName(name)}`));
    if (!names.length) {
      plans.push({ block, variants: [{ dialect: block.dialect, needs: [] }], expand: false });
      continue;
    }
    const needs = (dialect: Dialect): Need[] =>
      names.map((name) => {
        const found = resolve(name, block, dialect);
        if (!found) throw new Error(`${where(block.node)}: the ts twoslash block imports ./${name}, and the page has no ${DIALECT_NAME[dialect]} block with that file name (a code block titled ${name}, or a code tab of that name).`);
        return { name, block: found };
      });
    if (block.from !== 'default') {
      plans.push({ block, variants: [{ dialect: block.dialect, needs: needs(block.dialect) }], expand: false });
      continue;
    }
    // No dialect of its own: the block follows what it imports. A group of dialect tabs gives it
    // a panel per dialect; a lone block gives it that block's dialect.
    const nearest = names.map((name) => resolve(name, block, undefined)!).filter(Boolean);
    const grouped = nearest.filter((b) => b.group);
    if (grouped.length) {
      const dialects = new Set(grouped.flatMap((b) => statements.filter((s) => s.group === b.group && s.name === b.name).map((s) => s.dialect)));
      const variants = (Object.keys(DIALECT_NAME) as Dialect[]).filter((d) => dialects.has(d)).map((dialect) => ({ dialect, needs: needs(dialect) }));
      plans.push({ block, variants, expand: true });
      continue;
    }
    const dialect = nearest.length && nearest.every((b) => b.dialect === nearest[0]!.dialect) ? nearest[0]!.dialect : block.dialect;
    plans.push({ block, variants: [{ dialect, needs: needs(dialect) }], expand: false });
  }
  return plans;
}

/** lib/sql-modules.json: dialect, then file name, then the hash of the block's text, then the module. */
export type Modules = Partial<Record<Dialect, Record<string, Record<string, string>>>>;

/** Whether the block's own code declares or imports `db`. */
const declaresDb = (code: string) =>
  /\b(?:const|let|var|using|function|class)\s+(?:db\b|\{[^}]*\bdb\b[^}]*\}|\[[^\]]*\bdb\b[^\]]*\])/.test(code) ||
  /\bimport\s+(?:type\s+)?(?:db\b|\*\s+as\s+db\b|[^;]*\{[^}]*\bdb\b[^}]*\})/.test(code);

/** The block's code with its hidden setup above `// ---cut---`, for one dialect. */
export function withSetup(block: Block, variant: Variant, modules: Modules, file = 'the page'): string {
  const code = block.node.value;
  const files: string[] = [];
  for (const { name, block: sql } of variant.needs) {
    const module = modules[variant.dialect]?.[name]?.[sql.hash];
    if (module === undefined) {
      throw new Error(`${file}:${block.line}: the ts twoslash block imports ./${name}, and lib/sql-modules.json holds no module for the ${DIALECT_NAME[variant.dialect]} statement at line ${sql.line}; run \`nub run capture\` in site/ (with the scratch databases running), then build again.`);
    }
    files.push(`// @filename: ${declarationName(name)}\n${module}`);
  }
  const globals: string[] = [];
  if (/\bBun\.|\bfrom\s+["']bun(?::sqlite)?["']/.test(code)) globals.push('/// <reference types="bun" />');
  if (/\bD1Database\b/.test(code) && !/@cloudflare\/workers-types/.test(code)) globals.push('/// <reference types="@cloudflare/workers-types" />');
  if (variant.needs.length && !declaresDb(code)) globals.push(`declare const db: ${CLIENT[variant.dialect]};`);
  if (globals.length) files.push(`// @filename: globals.d.ts\n${globals.join('\n')}\n`);
  if (!files.length) return code;
  const title = attribute(block.node.meta, 'title');
  const own = code.includes('---cut---') ? code : `// ---cut---\n${code}`;
  const visible = code.includes('@filename') ? code : `// @filename: ${title && /\.[cm]?tsx?$/.test(title) ? title : 'main.ts'}\n${own}`;
  return files.join('') + visible;
}

const jsx = (name: string, attributes: [string, string | null][], children: MdNode[]): MdNode => ({
  type: 'mdxJsxFlowElement',
  name,
  attributes: attributes.map(([name, value]) => ({ type: 'mdxJsxAttribute', name, value })),
  children,
});

/** lib/sql-modules.json as the build reads it: the site's own copy, read again for every page so a new capture shows on the next render. */
export function readModules(site = process.cwd()): Modules {
  const file = path.join(site, 'lib', 'sql-modules.json');
  return existsSync(file) ? (JSON.parse(readFileSync(file, 'utf8')) as Modules) : {};
}

export function remarkMapper() {
  return (tree: MdNode, file: { path?: string; data?: Record<string, unknown> }) => {
    // fumadocs-mdx hands remark plugins the loader's compiler (its own remark-include uses it):
    // with the captured modules as a dependency of the page, `next dev` compiles the page again
    // after a capture, instead of keeping the error it showed while the capture was missing.
    const compiler = file.data?._compiler as { addDependency(file: string): void } | undefined;
    compiler?.addDependency(path.join(process.cwd(), 'lib', 'sql-modules.json'));
    const page = !file.path ? 'the page' : path.relative(process.cwd(), file.path).startsWith('..') ? file.path : path.relative(process.cwd(), file.path);
    let blocks: Block[];
    let plans: Plan[];
    try {
      blocks = collect(tree);
      plans = plan(blocks);
    } catch (e) {
      throw new Error(`${page}: ${(e as Error).message}`);
    }
    const modules = plans.some((p) => p.variants.some((v) => v.needs.length)) ? readModules() : {};
    for (const { block, variants, expand } of plans) {
      if (!expand) {
        block.node.value = withSetup(block, variants[0]!, modules, page);
        continue;
      }
      const panels = variants.map((variant) => jsx('Dialect', [['value', DIALECT_NAME[variant.dialect]]], [{ ...block.node, data: { ...block.node.data }, value: withSetup(block, variant, modules, page) }]));
      const index = block.parent.children!.indexOf(block.node);
      block.parent.children!.splice(index, 1, jsx('DialectTabs', [['follow', null]], panels));
    }
    // The dialect and the file name are read; Shiki and Twoslash do not know the attributes.
    for (const { node } of blocks) if (node.meta) node.meta = node.meta.replace(/(?:^|\s)(?:dialect|file)=(?:"[^"]*"|'[^']*'|\S+)/g, '').trim();
  };
}
