import type {
  AssociationConfig,
  Field,
  InheritanceConfig,
  LandoSchema,
  Model,
  Relation,
} from './types.ts';
import { FORMAT_VERSION } from './types.ts';
import { ERROR_CODES, type ValidationError, type ValidationResult } from './errors.ts';

/**
 * Validates a schema. Returns either `{ ok: true }` or a list of all
 * structured errors found. The validator is non-fatal: if it can keep
 * checking past one error, it does, so callers see the full picture in a
 * single pass.
 *
 * Pure function — no I/O, no globals. Safe to call from any context.
 */
export function validateSchema(s: LandoSchema): ValidationResult {
  const errors: ValidationError[] = [];

  if (s.formatVersion !== FORMAT_VERSION) {
    errors.push({
      code: ERROR_CODES.L_SCHEMA_FORMAT_VERSION,
      message: `formatVersion ${String(s.formatVersion)} does not match supported version ${FORMAT_VERSION}`,
      path: 'formatVersion',
    });
    // Continue — most other checks don't depend on the version field.
  }

  for (const [modelName, model] of Object.entries(s.models)) {
    validateModel(s, modelName, model, errors);
  }

  for (let i = 0; i < s.relations.length; i++) {
    const rel = s.relations[i];
    if (rel === undefined) continue;
    validateRelation(s, rel, `relations[${i}]`, errors);
  }

  return errors.length === 0 ? { ok: true } : { ok: false, errors };
}

function validateModel(
  s: LandoSchema,
  modelName: string,
  model: Model,
  errors: ValidationError[],
): void {
  const path = `models.${modelName}`;

  // Duplicate field names within a model.
  const seen = new Set<string>();
  for (const field of model.fields) {
    if (seen.has(field.name)) {
      errors.push({
        code: ERROR_CODES.L_SCHEMA_DUPLICATE_FIELD,
        message: `duplicate field name "${field.name}" in model "${modelName}"`,
        path: `${path}.fields`,
      });
    }
    seen.add(field.name);
  }

  // Each field's type references something that exists.
  for (let i = 0; i < model.fields.length; i++) {
    const f = model.fields[i];
    if (f === undefined) continue;
    validateFieldType(s, modelName, f, `${path}.fields[${i}]`, errors);
  }

  // Primary key fields must exist on the model.
  if (model.primaryKey.fields.length === 0) {
    errors.push({
      code: ERROR_CODES.L_SCHEMA_PRIMARY_KEY_INVALID,
      message: `model "${modelName}" has no primary key`,
      path: `${path}.primaryKey`,
    });
  } else {
    for (const fieldName of model.primaryKey.fields) {
      if (!seen.has(fieldName)) {
        errors.push({
          code: ERROR_CODES.L_SCHEMA_UNKNOWN_FIELD,
          message: `primary key references unknown field "${fieldName}" on model "${modelName}"`,
          path: `${path}.primaryKey`,
        });
      }
    }
  }

  // Unique + index field references.
  for (let i = 0; i < model.uniques.length; i++) {
    const u = model.uniques[i];
    if (u === undefined) continue;
    for (const fname of u.fields) {
      if (!seen.has(fname)) {
        errors.push({
          code: ERROR_CODES.L_SCHEMA_UNKNOWN_FIELD,
          message: `unique constraint references unknown field "${fname}" on model "${modelName}"`,
          path: `${path}.uniques[${i}]`,
        });
      }
    }
  }
  for (let i = 0; i < model.indexes.length; i++) {
    const idx = model.indexes[i];
    if (idx === undefined) continue;
    for (const fname of idx.fields) {
      if (!seen.has(fname)) {
        errors.push({
          code: ERROR_CODES.L_SCHEMA_UNKNOWN_FIELD,
          message: `index references unknown field "${fname}" on model "${modelName}"`,
          path: `${path}.indexes[${i}]`,
        });
      }
    }
  }

  if (model.polymorphism !== undefined) {
    if (model.polymorphism.kind === 'inheritance') {
      validateInheritance(model, model.polymorphism, `${path}.polymorphism`, errors);
    } else {
      validateAssociation(s, model, model.polymorphism, `${path}.polymorphism`, errors);
    }
  }
}

function validateFieldType(
  s: LandoSchema,
  modelName: string,
  field: Field,
  path: string,
  errors: ValidationError[],
): void {
  const t = field.type;
  if (t.kind === 'enum' && s.enums[t.enum] === undefined) {
    errors.push({
      code: ERROR_CODES.L_SCHEMA_UNKNOWN_ENUM,
      message: `field "${modelName}.${field.name}" references unknown enum "${t.enum}"`,
      path,
    });
  } else if (t.kind === 'relation') {
    const rel = s.relations.find((r) => r.name === t.relation);
    if (rel === undefined) {
      errors.push({
        code: ERROR_CODES.L_SCHEMA_RELATION_MISMATCH,
        message: `field "${modelName}.${field.name}" references unknown relation "${t.relation}"`,
        path,
      });
      return;
    }
    // The field name must be one of the two named sides of the relation.
    const expected =
      modelName === rel.from
        ? rel.forwardName
        : modelName === rel.to
          ? rel.reverseName
          : null;
    if (expected === null) {
      errors.push({
        code: ERROR_CODES.L_SCHEMA_RELATION_MISMATCH,
        message: `model "${modelName}" is neither side of relation "${rel.name}"`,
        path,
      });
    } else if (expected !== field.name) {
      errors.push({
        code: ERROR_CODES.L_SCHEMA_RELATION_MISMATCH,
        message: `field "${modelName}.${field.name}" should be named "${expected}" to match relation "${rel.name}"`,
        path,
      });
    }
  }
}

function validateRelation(
  s: LandoSchema,
  rel: Relation,
  path: string,
  errors: ValidationError[],
): void {
  const from = s.models[rel.from];
  const to = s.models[rel.to];
  if (from === undefined) {
    errors.push({
      code: ERROR_CODES.L_SCHEMA_UNKNOWN_MODEL,
      message: `relation "${rel.name}" references unknown model "${rel.from}"`,
      path: `${path}.from`,
    });
  }
  if (to === undefined) {
    errors.push({
      code: ERROR_CODES.L_SCHEMA_UNKNOWN_MODEL,
      message: `relation "${rel.name}" references unknown model "${rel.to}"`,
      path: `${path}.to`,
    });
  }
  if (from !== undefined) {
    const fromField = from.fields.find((f) => f.name === rel.forwardName);
    if (fromField === undefined) {
      errors.push({
        code: ERROR_CODES.L_SCHEMA_RELATION_MISMATCH,
        message: `relation "${rel.name}" expects forward field "${rel.forwardName}" on model "${rel.from}"`,
        path: `${path}.forwardName`,
      });
    }
    for (const col of rel.fkColumns) {
      const f = from.fields.find((ff) => ff.columnName === col);
      if (f === undefined) {
        errors.push({
          code: ERROR_CODES.L_SCHEMA_UNKNOWN_FIELD,
          message: `relation "${rel.name}" references unknown column "${col}" on model "${rel.from}"`,
          path: `${path}.fkColumns`,
        });
      }
    }
  }
  if (to !== undefined) {
    const toField = to.fields.find((f) => f.name === rel.reverseName);
    if (toField === undefined) {
      errors.push({
        code: ERROR_CODES.L_SCHEMA_RELATION_MISMATCH,
        message: `relation "${rel.name}" expects reverse field "${rel.reverseName}" on model "${rel.to}"`,
        path: `${path}.reverseName`,
      });
    }
    for (const col of rel.references) {
      const f = to.fields.find((ff) => ff.columnName === col);
      if (f === undefined) {
        errors.push({
          code: ERROR_CODES.L_SCHEMA_UNKNOWN_FIELD,
          message: `relation "${rel.name}" references unknown column "${col}" on model "${rel.to}"`,
          path: `${path}.references`,
        });
      }
    }
  }
}

function validateInheritance(
  model: Model,
  cfg: InheritanceConfig,
  path: string,
  errors: ValidationError[],
): void {
  const discriminator = model.fields.find((f) => f.columnName === cfg.discriminator);
  if (discriminator === undefined) {
    errors.push({
      code: ERROR_CODES.L_SCHEMA_INHERITANCE_DISCRIMINATOR_MISSING,
      message: `inheritance polymorphism on "${model.name}" references unknown discriminator column "${cfg.discriminator}"`,
      path: `${path}.discriminator`,
    });
  }

  const variantNames = Object.keys(cfg.variants);
  if (variantNames.length === 0) {
    errors.push({
      code: ERROR_CODES.L_SCHEMA_INHERITANCE_VARIANT_INVALID,
      message: `inheritance polymorphism on "${model.name}" declares no variants`,
      path: `${path}.variants`,
    });
  }

  // For single-table strategy, every `include` field must exist on the model.
  if (cfg.strategy === 'single-table') {
    const fieldNames = new Set(model.fields.map((f) => f.name));
    for (const [variantKey, spec] of Object.entries(cfg.variants)) {
      for (const fname of spec.include ?? []) {
        if (!fieldNames.has(fname)) {
          errors.push({
            code: ERROR_CODES.L_SCHEMA_INHERITANCE_VARIANT_INVALID,
            message: `variant "${variantKey}" of "${model.name}" references unknown field "${fname}"`,
            path: `${path}.variants.${variantKey}.include`,
          });
        }
      }
    }
  }
}

function validateAssociation(
  s: LandoSchema,
  model: Model,
  cfg: AssociationConfig,
  path: string,
  errors: ValidationError[],
): void {
  const variantEntries = Object.entries(cfg.variants);
  if (variantEntries.length < 2) {
    errors.push({
      code: ERROR_CODES.L_SCHEMA_ASSOCIATION_TOO_FEW_VARIANTS,
      message: `polymorphic association on "${model.name}" needs at least two variants; found ${variantEntries.length}`,
      path: `${path}.variants`,
    });
  }
  for (const [key, spec] of variantEntries) {
    const column = model.fields.find((f) => f.columnName === spec.column);
    if (column === undefined) {
      errors.push({
        code: ERROR_CODES.L_SCHEMA_ASSOCIATION_VARIANT_INVALID,
        message: `variant "${key}" on "${model.name}" references unknown column "${spec.column}"`,
        path: `${path}.variants.${key}.column`,
      });
    } else if (!column.nullable) {
      errors.push({
        code: ERROR_CODES.L_SCHEMA_ASSOCIATION_VARIANT_INVALID,
        message: `variant "${key}" on "${model.name}" requires column "${spec.column}" to be nullable`,
        path: `${path}.variants.${key}.column`,
      });
    }
    if (s.models[spec.references] === undefined) {
      errors.push({
        code: ERROR_CODES.L_SCHEMA_UNKNOWN_MODEL,
        message: `variant "${key}" on "${model.name}" references unknown model "${spec.references}"`,
        path: `${path}.variants.${key}.references`,
      });
    }
  }
}
