// Hand-rolled validator for `lando.config.ts`. Zod would be cleaner but
// would add a runtime dep just for this; the surface is small enough
// that the savings aren't worth it. Keep this file boring and
// exhaustive — config validation is the first error a new user sees,
// and bad messages here cost more than they look like they should.

import type { Adapter } from '#schema';
import { CONFIG_ERROR_CODES, type ConfigError } from './errors.ts';
import type { LandoConfig, SchemaSource } from './types.ts';

export type ValidateConfigResult =
  | { ok: true; config: LandoConfig }
  | { ok: false; errors: ConfigError[] };

export function validateConfig(value: unknown): ValidateConfigResult {
  const errors: ConfigError[] = [];

  if (!isPlainObject(value)) {
    return {
      ok: false,
      errors: [
        {
          code: CONFIG_ERROR_CODES.L_CONFIG_INVALID_SHAPE,
          message: 'config must be an object literal',
          path: '',
        },
      ],
    };
  }

  const source = validateSource(value['source'], errors);
  const adapter = validateAdapter(value['adapter'], errors);
  validatePolymorphism(value['polymorphism'], errors);
  validateIntrospectAt(value['introspectAt'], errors);
  validateValidate(value['validate'], errors);

  if (errors.length > 0) return { ok: false, errors };

  // The narrowing above guarantees these are present on the success
  // path; the casts express that to TS without re-validating.
  const config: LandoConfig = {
    source: source as SchemaSource,
    adapter: adapter as Adapter,
  };
  const polymorphism = value['polymorphism'];
  if (isPlainObject(polymorphism)) {
    config.polymorphism = polymorphism as NonNullable<LandoConfig['polymorphism']>;
  }
  const introspectAt = value['introspectAt'];
  if (introspectAt === 'build' || introspectAt === 'runtime') {
    config.introspectAt = introspectAt;
  }
  const validate = value['validate'];
  if (isPlainObject(validate) && (validate['input'] === 'on' || validate['input'] === 'off')) {
    config.validate = { input: validate['input'] };
  }
  return { ok: true, config };
}

// Sub-validators -------------------------------------------------------

function validateSource(value: unknown, errors: ConfigError[]): SchemaSource | undefined {
  if (!isPlainObject(value)) {
    errors.push({
      code: CONFIG_ERROR_CODES.L_CONFIG_INVALID_SOURCE,
      message: 'source must be an object',
      path: 'source',
    });
    return undefined;
  }
  const kind = value['kind'];
  if (kind === 'prisma') {
    const path = value['path'];
    if (typeof path !== 'string' || path.length === 0) {
      errors.push({
        code: CONFIG_ERROR_CODES.L_CONFIG_INVALID_SOURCE,
        message: 'source.path must be a non-empty string for kind="prisma"',
        path: 'source.path',
      });
      return undefined;
    }
    return { kind: 'prisma', path };
  }
  if (kind === 'introspection') {
    const url = value['url'];
    if (typeof url !== 'string' || url.length === 0) {
      errors.push({
        code: CONFIG_ERROR_CODES.L_CONFIG_INVALID_SOURCE,
        message: 'source.url must be a non-empty string for kind="introspection"',
        path: 'source.url',
      });
      return undefined;
    }
    return { kind: 'introspection', url };
  }
  errors.push({
    code: CONFIG_ERROR_CODES.L_CONFIG_INVALID_SOURCE,
    message: `source.kind must be "prisma" or "introspection", got ${JSON.stringify(kind)}`,
    path: 'source.kind',
  });
  return undefined;
}

function validateAdapter(value: unknown, errors: ConfigError[]): Adapter | undefined {
  if (!isPlainObject(value)) {
    errors.push({
      code: CONFIG_ERROR_CODES.L_CONFIG_INVALID_ADAPTER,
      message: 'adapter must be the result of calling an adapter factory (e.g. pg())',
      path: 'adapter',
    });
    return undefined;
  }
  if (typeof value['kind'] !== 'string') {
    errors.push({
      code: CONFIG_ERROR_CODES.L_CONFIG_INVALID_ADAPTER,
      message: 'adapter.kind must be a string identifying the engine ("pg", "sqlite", ...)',
      path: 'adapter.kind',
    });
    return undefined;
  }
  if (!isPlainObject(value['capabilities'])) {
    errors.push({
      code: CONFIG_ERROR_CODES.L_CONFIG_INVALID_ADAPTER,
      message: 'adapter.capabilities must be an object',
      path: 'adapter.capabilities',
    });
    return undefined;
  }
  return value as unknown as Adapter;
}

function validatePolymorphism(value: unknown, errors: ConfigError[]): void {
  if (value === undefined) return;
  if (!isPlainObject(value)) {
    errors.push({
      code: CONFIG_ERROR_CODES.L_CONFIG_INVALID_POLYMORPHISM,
      message: 'polymorphism must be a record keyed by model name',
      path: 'polymorphism',
    });
    return;
  }
  for (const [modelName, cfg] of Object.entries(value)) {
    if (!isPlainObject(cfg) || (cfg['kind'] !== 'inheritance' && cfg['kind'] !== 'association')) {
      errors.push({
        code: CONFIG_ERROR_CODES.L_CONFIG_INVALID_POLYMORPHISM,
        message: `polymorphism["${modelName}"].kind must be "inheritance" or "association"`,
        path: `polymorphism.${modelName}.kind`,
      });
    }
    // Deeper structural validation happens after the schema is built —
    // that's where we know which models / columns / fields exist.
    // (See `applyPolymorphism` in `src/prisma/polymorphism.ts`.)
  }
}

function validateIntrospectAt(value: unknown, errors: ConfigError[]): void {
  if (value === undefined) return;
  if (value !== 'build' && value !== 'runtime') {
    errors.push({
      code: CONFIG_ERROR_CODES.L_CONFIG_INVALID_SHAPE,
      message: 'introspectAt must be "build" or "runtime" when set',
      path: 'introspectAt',
    });
  }
}

function validateValidate(value: unknown, errors: ConfigError[]): void {
  if (value === undefined) return;
  if (!isPlainObject(value) || (value['input'] !== 'on' && value['input'] !== 'off')) {
    errors.push({
      code: CONFIG_ERROR_CODES.L_CONFIG_INVALID_SHAPE,
      message: 'validate must be { input: "on" | "off" } when set',
      path: 'validate',
    });
  }
}

function isPlainObject(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}
