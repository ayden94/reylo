// Postgres type OID → TypeScript type, for the pg / postgres decoders (both parse
// int8 and numeric as strings and timestamps as Date). Arrays, enums and unknown
// OIDs are resolved through pg_type by the caller.
const scalar: Record<number, string> = {
  16: "boolean", // bool
  20: "string", // int8
  21: "number", // int2
  23: "number", // int4
  26: "number", // oid
  700: "number", // float4
  701: "number", // float8
  1700: "string", // numeric
  25: "string", // text
  1043: "string", // varchar
  1042: "string", // bpchar
  19: "string", // name
  2950: "string", // uuid
  1082: "Date", // date
  1114: "Date", // timestamp
  1184: "Date", // timestamptz
  1083: "string", // time
  1186: "string", // interval
  114: "unknown", // json
  3802: "unknown", // jsonb
  17: "Uint8Array", // bytea
  2278: "void", // void
};

export function scalarType(oid: number): string | undefined {
  return scalar[oid];
}

export type PgTypeRow = { oid: number; typname: string; typtype: string; typelem: number; typcategory: string; labels: string[] | null };

/** Resolve an OID to TypeScript text, asking the catalog for arrays, enums, and domains. */
export async function typeText(oid: number, lookup: (oids: number[]) => Promise<PgTypeRow[]>): Promise<string> {
  const direct = scalarType(oid);
  if (direct) return direct;
  const [row] = await lookup([oid]);
  if (!row) return "unknown";
  if (row.typtype === "e") return row.labels && row.labels.length ? row.labels.map((l) => JSON.stringify(l)).join(" | ") : "string";
  if (row.typcategory === "A" && row.typelem) {
    const inner = await typeText(row.typelem, lookup);
    return /[|&]/.test(inner) ? `(${inner})[]` : `${inner}[]`;
  }
  return "unknown";
}

export const PG_TYPE_QUERY = `select t.oid::int as oid, t.typname, t.typtype::text as typtype, t.typelem::int as typelem, t.typcategory::text as typcategory,
  (select array_agg(e.enumlabel order by e.enumsortorder) from pg_enum e where e.enumtypid = t.oid) as labels
  from pg_type t where t.oid = any($1::oid[])`;
