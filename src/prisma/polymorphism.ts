// Polymorphism overlay. Pure function: takes a freshly-lowered schema
// plus the user's `polymorphism` block from `lando.config.ts` and
// returns a new schema with `PolymorphismConfig` attached to the
// relevant models. Validation runs over the merged result; any
// reference to a missing model / column / field surfaces as a
// structured error.
//
// Phase 3 (the sidecar config loader) calls this; we keep it stateless
// here so it stays testable in isolation without touching the disk.

import type {
  AssociationConfig,
  InheritanceConfig,
  LandoSchema,
  PolymorphismConfig,
  ValidationError,
  ValidationResult,
} from '#schema';
import { ERROR_CODES, validateSchema } from '#schema';

export type PolymorphismOverlay = Record<string, PolymorphismConfig>;

export type ApplyPolymorphismResult =
  | { ok: true; schema: LandoSchema }
  | { ok: false; errors: ValidationError[] };

/**
 * Attaches polymorphism configs to their target models and validates
 * the merged schema. Returns the merged schema or a list of errors.
 *
 * Pure: never mutates `schema` or `overlay`. Safe to call from anywhere.
 */
export function applyPolymorphism(
  schema: LandoSchema,
  overlay: PolymorphismOverlay,
): ApplyPolymorphismResult {
  const errors: ValidationError[] = [];
  const nextModels = { ...schema.models };

  for (const [modelName, cfg] of Object.entries(overlay)) {
    const model = nextModels[modelName];
    if (model === undefined) {
      errors.push({
        code: ERROR_CODES.L_SCHEMA_UNKNOWN_MODEL,
        message: `polymorphism overlay references unknown model "${modelName}"`,
        path: `polymorphism.${modelName}`,
      });
      continue;
    }
    if (model.polymorphism !== undefined) {
      errors.push({
        code: errorCodeFor(cfg),
        message: `model "${modelName}" already has a polymorphism config; overlay would conflict`,
        path: `polymorphism.${modelName}`,
      });
      continue;
    }
    nextModels[modelName] = { ...model, polymorphism: cfg };
  }

  const merged: LandoSchema = { ...schema, models: nextModels };

  // Defer detail-level integrity checks to `validateSchema` so the rules
  // stay in one place. We layer the overlay-specific errors on top.
  const validation: ValidationResult = validateSchema(merged);
  if (!validation.ok) errors.push(...validation.errors);

  return errors.length === 0 ? { ok: true, schema: merged } : { ok: false, errors };
}

function errorCodeFor(
  cfg: PolymorphismConfig,
): typeof ERROR_CODES.L_SCHEMA_INHERITANCE_VARIANT_INVALID | typeof ERROR_CODES.L_SCHEMA_ASSOCIATION_VARIANT_INVALID {
  return cfg.kind === 'inheritance'
    ? ERROR_CODES.L_SCHEMA_INHERITANCE_VARIANT_INVALID
    : ERROR_CODES.L_SCHEMA_ASSOCIATION_VARIANT_INVALID;
}

// Re-export for consumers writing config files. These already exist on
// `#schema`; re-exporting from here lets `lando.config.ts` import
// everything it needs from one place.
export type { AssociationConfig, InheritanceConfig, PolymorphismConfig };
