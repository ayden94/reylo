import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { getSchema, type Schema as PrismaAst } from '@mrleebo/prisma-ast';

/**
 * Parses a Prisma schema *file* off disk. Returns the AST and a small
 * envelope describing the file's datasource and any generator blocks
 * (we don't ship a generator, but we surface the URL hint so that the
 * BYO-DB workflow can pick it up — see whitepaper §"Schema source").
 *
 * `parsePrismaFile` is the only function in this module that touches
 * the filesystem; tests should prefer `parsePrismaSource`.
 */
export function parsePrismaFile(path: string): ParsedPrisma {
  const absolute = resolve(path);
  const source = readFileSync(absolute, 'utf8');
  return { ...parsePrismaSource(source), path: absolute };
}

export function parsePrismaSource(source: string): ParsedPrisma {
  const ast = getSchema(source);
  const datasource = readDatasource(ast);
  const generators = readGenerators(ast);
  return { ast, datasource, generators, path: undefined };
}

export type ParsedPrisma = {
  ast: PrismaAst;
  /** Absolute filesystem path, when available. */
  path: string | undefined;
  /** First `datasource` block; later blocks are ignored (Prisma allows one). */
  datasource: DatasourceInfo | undefined;
  generators: GeneratorInfo[];
};

export type DatasourceInfo = {
  name: string;
  provider: string;
  /** Literal URL, or `env("VAR_NAME")`, recorded as `{ env: 'VAR_NAME' }`. */
  url: { kind: 'literal'; value: string } | { kind: 'env'; name: string };
};

export type GeneratorInfo = {
  name: string;
  provider: string;
  output?: string;
};

// AST helpers — narrow to just what we need from each block. The types
// `@mrleebo/prisma-ast` exports are wider than we want to consume here.

function readDatasource(ast: PrismaAst): DatasourceInfo | undefined {
  for (const node of ast.list) {
    if (node.type !== 'datasource') continue;
    const provider = readScalar(node.assignments, 'provider');
    const urlNode = readAssignment(node.assignments, 'url');
    const url = parseUrl(urlNode);
    if (provider === undefined || url === undefined) continue;
    return { name: node.name, provider, url };
  }
  return undefined;
}

function readGenerators(ast: PrismaAst): GeneratorInfo[] {
  const out: GeneratorInfo[] = [];
  for (const node of ast.list) {
    if (node.type !== 'generator') continue;
    const provider = readScalar(node.assignments, 'provider');
    if (provider === undefined) continue;
    const output = readScalar(node.assignments, 'output');
    out.push(output === undefined ? { name: node.name, provider } : { name: node.name, provider, output });
  }
  return out;
}

type Assignments = ReadonlyArray<{ type: string; key?: string; value?: unknown }>;

function readAssignment(assignments: Assignments, key: string): unknown {
  for (const a of assignments) {
    if (a.type === 'assignment' && a.key === key) return a.value;
  }
  return undefined;
}

function readScalar(assignments: Assignments, key: string): string | undefined {
  const v = readAssignment(assignments, key);
  return typeof v === 'string' ? stripQuotes(v) : undefined;
}

function parseUrl(value: unknown): DatasourceInfo['url'] | undefined {
  if (typeof value === 'string') return { kind: 'literal', value: stripQuotes(value) };
  if (value !== null && typeof value === 'object' && 'type' in value && value.type === 'function') {
    const fn = value as unknown as { name: string; params?: string[] };
    if (fn.name === 'env' && fn.params && fn.params.length >= 1) {
      const first = fn.params[0];
      if (first !== undefined) return { kind: 'env', name: stripQuotes(first) };
    }
  }
  return undefined;
}

function stripQuotes(s: string): string {
  if (s.length >= 2 && s.startsWith('"') && s.endsWith('"')) return s.slice(1, -1);
  return s;
}
