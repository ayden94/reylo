import type { LandoSchema, Model } from './types.ts';

/**
 * Returns a structurally identical schema with deterministic ordering for
 * collections whose order is *not* semantically meaningful. Used both
 * before printing (so artifacts have stable diffs) and before checksum
 * computation.
 *
 * Order rules:
 *   - `models`, `enums`           — keys sorted alphabetically.
 *   - `model.uniques`,
 *     `model.indexes`,
 *     `relations`                 — sorted by `name` (or by joined fields
 *                                   when `name` is absent).
 *   - `model.fields`,
 *     `enum.values`               — preserved as given.
 *
 * The function is pure; it never mutates its input.
 */
export function canonicalize(s: LandoSchema): LandoSchema {
  return {
    formatVersion: s.formatVersion,
    models: sortRecord(s.models, canonicalizeModel),
    enums: sortRecord(s.enums, (e) => ({ name: e.name, values: [...e.values] })),
    relations: [...s.relations]
      .map((r) => ({ ...r, fkColumns: [...r.fkColumns], references: [...r.references] }))
      .sort((a, b) => a.name.localeCompare(b.name)),
    metadata: { ...s.metadata },
  };
}

function canonicalizeModel(m: Model): Model {
  const out: Model = {
    name: m.name,
    tableName: m.tableName,
    fields: m.fields.map((f) => ({ ...f, attributes: { ...f.attributes } })),
    primaryKey: { fields: [...m.primaryKey.fields] },
    uniques: [...m.uniques]
      .map((u) => ({ ...u, fields: [...u.fields] }))
      .sort(byNameOrFields),
    indexes: [...m.indexes]
      .map((i) => ({ ...i, fields: [...i.fields] }))
      .sort(byNameOrFields),
  };
  if (m.polymorphism !== undefined) {
    out.polymorphism = m.polymorphism;
  }
  return out;
}

function byNameOrFields(
  a: { name?: string; fields: string[] },
  b: { name?: string; fields: string[] },
): number {
  const an = a.name ?? a.fields.join(',');
  const bn = b.name ?? b.fields.join(',');
  return an.localeCompare(bn);
}

function sortRecord<T, U>(
  rec: Record<string, T>,
  map: (value: T, key: string) => U,
): Record<string, U> {
  const out: Record<string, U> = {};
  for (const key of Object.keys(rec).sort()) {
    const v = rec[key];
    if (v === undefined) continue;
    out[key] = map(v, key);
  }
  return out;
}
