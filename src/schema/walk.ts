import type { Field, LandoSchema, Model, PolymorphismConfig, Relation } from './types.ts';

/** Returns the model with the given name, or `undefined`. */
export function getModel(s: LandoSchema, name: string): Model | undefined {
  return s.models[name];
}

/** Returns the enum with the given name, or `undefined`. */
export function getEnum(s: LandoSchema, name: string): { name: string; values: string[] } | undefined {
  return s.enums[name];
}

/**
 * Looks up the relation owning the given field. Either side of the
 * relation matches: passing the forward field returns the relation, and
 * passing the reverse field returns the same relation. Returns
 * `undefined` if no such relation exists.
 */
export function getRelation(
  s: LandoSchema,
  modelName: string,
  fieldName: string,
): Relation | undefined {
  for (const r of s.relations) {
    if (r.from === modelName && r.forwardName === fieldName) return r;
    if (r.to === modelName && r.reverseName === fieldName) return r;
  }
  return undefined;
}

/** Returns the polymorphism config attached to the given model, if any. */
export function getPolymorphism(
  s: LandoSchema,
  modelName: string,
): PolymorphismConfig | undefined {
  return s.models[modelName]?.polymorphism;
}

/** Iterates every (model, field) pair in declaration order. */
export function forEachField(s: LandoSchema, fn: (model: Model, field: Field) => void): void {
  for (const modelName of Object.keys(s.models)) {
    const model = s.models[modelName];
    if (model === undefined) continue;
    for (const field of model.fields) fn(model, field);
  }
}

/** Iterates every relation in the schema. */
export function forEachRelation(s: LandoSchema, fn: (relation: Relation) => void): void {
  for (const r of s.relations) fn(r);
}
