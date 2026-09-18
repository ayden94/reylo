// The engine's type model as TypeScript text. On the wire a driver decodes a Postgres value
// its own way (int8 as a string, timestamps as Date); inside a JSON value the database has
// already serialized it, so the same type reads differently there.
import type { EngineType } from "./engine.ts";

const wire: Record<string, string> = {
  bool: "boolean",
  int2: "number", int4: "number", oid: "number", float4: "number", float8: "number",
  int8: "string", numeric: "string",
  text: "string", varchar: "string", bpchar: "string", name: "string", char: "string", citext: "string",
  uuid: "string", time: "string", timetz: "string", interval: "string",
  date: "Date", timestamp: "Date", timestamptz: "Date",
  bytea: "Uint8Array",
  void: "void",
};
const json: Record<string, string> = {
  ...wire,
  int8: "number", numeric: "number",
  date: "string", timestamp: "string", timestamptz: "string",
  bytea: "string",
};

export const ident = (name: string) => (/^[A-Za-z_$][\w$]*$/.test(name) ? name : JSON.stringify(name));

export function renderType(t: EngineType, inJson = false): string {
  switch (t.kind) {
    case "scalar": return (inJson ? json : wire)[t.name] ?? "unknown";
    case "enum": return t.labels.length ? t.labels.map((l) => JSON.stringify(l)).join(" | ") : "string";
    case "array": { const inner = renderType(t.of, inJson); return /[|&]/.test(inner) ? `(${inner})[]` : `${inner}[]`; }
    case "object": return t.fields.length ? `{ ${t.fields.map((f) => `${ident(f.name)}: ${renderType(f.type, true)}${f.nullable ? " | null" : ""}`).join("; ")} }` : "Record<never, never>";
    case "list": { const inner = renderType(t.of, true) + (t.nullable_items ? " | null" : ""); return /[|&]/.test(inner) ? `(${inner})[]` : `${inner}[]`; }
    case "json": return "unknown";
    case "unknown": return "unknown";
  }
}
