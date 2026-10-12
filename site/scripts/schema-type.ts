// The TypeScript type of a typed JSON parameter, from the schema in its cast
// (`:users::jsonb('{ ... }')`, docs/typed-json-params.md). The loader types such a parameter
// `unknown` until typed JSON parameters are built; scripts/capture-types.ts puts this type in its
// place, so the docs show what the build will give.
//
// One function reads both notations the maintainer is choosing between:
// - the page's: JSON Schema's shape with the dialect's type names as leaves (`"name": "text"`), a
//   bare string as a property's schema, a key ending in `?` optional and nullable
//   (`email?: string | null`), and every other key required;
// - plain JSON Schema: `string`, `integer`, `number`, `boolean`, `null`, `object` and `array`,
//   `required`, `"type": [..., "null"]`, `enum`, `const`, `oneOf` and `anyOf`.
// A schema is read as the page's notation when anything in it is (a bare string as a schema, a
// key ending in `?`, a dialect's type name); otherwise as JSON Schema, where a property that
// `required` does not list is optional. An object with a `required` list follows the list in
// either notation. A type name the dialect does not have is an error.

export type Dialect = 'postgres' | 'mysql' | 'sqlite';
type Schema = string | boolean | { [key: string]: unknown };

const JSON_SCHEMA_TYPES = new Set(['string', 'integer', 'number', 'boolean', 'null', 'object', 'array']);

const names = (type: string, words: string) => Object.fromEntries(words.split(',').map((w) => [w.trim(), type]));

// Each dialect's type names, as a JSON value carries them: inside JSON a timestamp, a uuid, a
// decimal or a 64-bit integer is a string or a number, never a Date or a BigInt; binary is text
// (Postgres `\x` hex, a field the statement decodes with unhex() elsewhere); a json value is
// whatever JSON, as the loader renders a JSON column (`unknown`).
const LEAVES: Record<Dialect, Record<string, string>> = {
  postgres: {
    ...names('string', 'text, varchar, character varying, char, character, bpchar, citext, name, uuid, date, time, timetz, time with time zone, time without time zone, timestamp, timestamptz, timestamp with time zone, timestamp without time zone, interval, bytea, inet, cidr, macaddr, macaddr8, xml, money'),
    ...names('number', 'smallint, int2, integer, int, int4, bigint, int8, real, float4, double precision, float8, float, numeric, decimal, smallserial, serial2, serial, serial4, bigserial, serial8, oid'),
    ...names('boolean', 'boolean, bool'),
    ...names('unknown', 'json, jsonb'),
  },
  mysql: {
    ...names('string', 'char, varchar, character, binary, varbinary, tinytext, text, mediumtext, longtext, tinyblob, blob, mediumblob, longblob, date, datetime, timestamp, time'),
    ...names('number', 'tinyint, smallint, mediumint, int, integer, bigint, decimal, dec, numeric, fixed, float, double, double precision, real, year'),
    ...names('boolean', 'bool, boolean'),
    ...names('unknown', 'json'),
  },
  // The names a strict table takes, and json.
  sqlite: {
    ...names('number', 'int, integer, real'),
    ...names('string', 'text, blob'),
    ...names('unknown', 'any, json'),
  },
};

/** A dialect's type name as TypeScript: `varchar(255)`, `int unsigned`, `text[]`. */
function leaf(name: string, dialect: Dialect, at: string): string {
  let base = name.trim().toLowerCase().replace(/\s+/g, ' ');
  let dims = 0;
  while (dialect === 'postgres' && base.endsWith('[]')) {
    base = base.slice(0, -2).trim();
    dims++;
  }
  base = base.replace(/\s*\([^)]*\)/g, '');
  if (dialect === 'mysql') base = base.replace(/(?:\s+(?:unsigned|signed|zerofill))+$/, '');
  const type = JSON_SCHEMA_TYPES.has(base) ? jsonSchemaType(base) : LEAVES[dialect][base];
  if (type === undefined) throw new Error(`${at}: "${name}" is not a ${dialect === 'postgres' ? 'Postgres' : dialect === 'mysql' ? 'MySQL' : 'SQLite'} type name this schema reads (nor one of JSON Schema's: ${[...JSON_SCHEMA_TYPES].join(', ')})`);
  return dims ? `${parens(type)}${'[]'.repeat(dims)}` : type;
}

function jsonSchemaType(name: string): string {
  return name === 'integer' ? 'number' : name === 'object' ? 'Record<string, unknown>' : name === 'array' ? 'unknown[]' : name;
}

const ident = (name: string) => (/^[A-Za-z_$][\w$]*$/.test(name) ? name : JSON.stringify(name));
/** A type as an array's element or an intersection's member: in parentheses when it is a union or an intersection. */
const parens = (type: string) => (topLevel(type, ' | ').length > 1 || topLevel(type, ' & ').length > 1 ? `(${type})` : type);
const union = (types: string[]) => {
  const parts = [...new Set(types.flatMap((t) => topLevel(t, ' | ')))];
  if (parts.includes('unknown')) return 'unknown';
  return parts.join(' | ') || 'never';
};
/** The parts of a type around a separator at its top level, never inside braces, brackets or a string. */
function topLevel(type: string, separator: string): string[] {
  const parts: string[] = [];
  let depth = 0;
  let start = 0;
  for (let i = 0; i < type.length; i++) {
    const c = type[i];
    if (c === '{' || c === '[' || c === '(' || c === '<') depth++;
    else if (c === '}' || c === ']' || c === ')' || c === '>') depth--;
    else if (c === '"') i = skipString(type, i);
    else if (depth === 0 && type.startsWith(separator, i)) {
      parts.push(type.slice(start, i));
      start = i + separator.length;
      i += separator.length - 1;
    }
  }
  parts.push(type.slice(start));
  return parts.map((p) => p.trim()).filter(Boolean);
}
function skipString(text: string, i: number): number {
  for (let j = i + 1; j < text.length; j++) {
    if (text[j] === '\\') j++;
    else if (text[j] === '"') return j;
  }
  return text.length;
}

/** Whether the schema uses the page's notation anywhere: a bare string as a schema, a key ending in `?`, a type name of the dialect. */
function pageNotation(schema: unknown): boolean {
  if (typeof schema === 'string') return true;
  if (Array.isArray(schema)) return schema.some(pageNotation);
  if (schema === null || typeof schema !== 'object') return false;
  const s = schema as Record<string, unknown>;
  const types = typeof s.type === 'string' ? [s.type] : Array.isArray(s.type) ? s.type : [];
  if (types.some((t) => typeof t === 'string' && !JSON_SCHEMA_TYPES.has(t))) return true;
  const props = s.properties && typeof s.properties === 'object' ? (s.properties as Record<string, unknown>) : {};
  if (Object.keys(props).some((k) => k.endsWith('?'))) return true;
  return [...Object.values(props), s.items, s.additionalProperties, ...(['oneOf', 'anyOf', 'allOf', 'prefixItems'] as const).flatMap((k) => (Array.isArray(s[k]) ? (s[k] as unknown[]) : []))].some((v) => v !== undefined && typeof v !== 'boolean' && pageNotation(v));
}

/**
 * The TypeScript type of a value the schema accepts. `dialect` reads its type names; `at` names
 * the place for an error (the parameter).
 */
export function schemaType(schema: Schema, dialect: Dialect, at = 'the schema'): string {
  const page = pageNotation(schema);
  const type = (s: unknown, where: string): string => {
    if (s === true || (s !== null && typeof s === 'object' && !Array.isArray(s) && Object.keys(s).length === 0)) return 'unknown';
    if (s === false) return 'never';
    if (typeof s === 'string') return leaf(s, dialect, where);
    if (s === null || typeof s !== 'object' || Array.isArray(s)) throw new Error(`${where}: a schema is an object or a type name, not ${JSON.stringify(s)}`);
    const o = s as Record<string, unknown>;
    if ('const' in o) return JSON.stringify(o.const);
    if (Array.isArray(o.enum)) return union(o.enum.map((v) => JSON.stringify(v)));
    const alternatives = (['oneOf', 'anyOf'] as const).find((k) => Array.isArray(o[k]));
    const own = Object.keys(o).some((k) => ['type', 'properties', 'items', 'prefixItems', 'additionalProperties', 'required'].includes(k));
    if (alternatives) {
      const members = union((o[alternatives] as unknown[]).map((m, i) => type(m, `${where}.${alternatives}[${i}]`)));
      if (!own) return members;
      const { [alternatives]: _, ...base } = o;
      return `${parens(type(base, where))} & (${members})`;
    }
    if (Array.isArray(o.allOf)) return (o.allOf as unknown[]).map((m, i) => parens(type(m, `${where}.allOf[${i}]`))).join(' & ');
    const types = typeof o.type === 'string' ? [o.type] : Array.isArray(o.type) ? (o.type as unknown[]) : o.properties || o.additionalProperties !== undefined ? ['object'] : o.items || o.prefixItems ? ['array'] : [];
    if (!types.length) return 'unknown';
    return union(
      types.map((t) => {
        if (typeof t !== 'string') throw new Error(`${where}: "type" holds ${JSON.stringify(t)}, not a type name`);
        if (t === 'object') return object(o, where);
        if (t === 'array') return array(o, where);
        return leaf(t, dialect, where);
      }),
    );
  };
  const object = (o: Record<string, unknown>, where: string): string => {
    const props = (o.properties ?? {}) as Record<string, unknown>;
    if (typeof props !== 'object' || Array.isArray(props)) throw new Error(`${where}: "properties" is an object of schemas`);
    const required = Array.isArray(o.required) ? new Set(o.required as string[]) : undefined;
    const fields = Object.entries(props).map(([key, value]) => {
      const loose = key.endsWith('?');
      const name = loose ? key.slice(0, -1) : key;
      const t = type(value, `${where}.${name}`);
      if (loose) return `${ident(name)}?: ${union([t, 'null'])}`;
      const optional = required ? !required.has(name) : !page;
      return `${ident(name)}${optional ? '?' : ''}: ${t}`;
    });
    const extra = o.additionalProperties;
    const rest = extra === undefined || extra === false ? undefined : extra === true ? 'unknown' : type(extra, `${where}.additionalProperties`);
    if (!fields.length) return rest === undefined ? (extra === false ? 'Record<never, never>' : 'Record<string, unknown>') : `Record<string, ${rest}>`;
    const shape = `{ ${fields.join('; ')} }`;
    return rest === undefined ? shape : `${shape} & Record<string, ${rest}>`;
  };
  const array = (o: Record<string, unknown>, where: string): string => {
    if (Array.isArray(o.prefixItems)) return `[${(o.prefixItems as unknown[]).map((m, i) => type(m, `${where}[${i}]`)).join(', ')}]`;
    if (o.items === undefined) return 'unknown[]';
    return `${parens(type(o.items, `${where}[]`))}[]`;
  };
  return type(schema, at);
}
