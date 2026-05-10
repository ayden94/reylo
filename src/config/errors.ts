// Error codes for config loading + validation. Same convention as the
// schema codes (`L_SCHEMA_*`): append-only, stable across versions.

export const CONFIG_ERROR_CODES = {
  L_CONFIG_NOT_FOUND: 'L_CONFIG_001',
  L_CONFIG_LOAD_FAILED: 'L_CONFIG_002',
  L_CONFIG_NO_DEFAULT_EXPORT: 'L_CONFIG_003',
  L_CONFIG_INVALID_SHAPE: 'L_CONFIG_004',
  L_CONFIG_INVALID_SOURCE: 'L_CONFIG_005',
  L_CONFIG_INVALID_ADAPTER: 'L_CONFIG_006',
  L_CONFIG_INVALID_POLYMORPHISM: 'L_CONFIG_007',
  L_CONFIG_PROJECT_META: 'L_CONFIG_008',
} as const;

export type ConfigErrorCode = (typeof CONFIG_ERROR_CODES)[keyof typeof CONFIG_ERROR_CODES];

export type ConfigError = {
  code: ConfigErrorCode;
  message: string;
  /** Dotted path into the config object, e.g. `source.url`. */
  path: string;
  /** Underlying cause, when the error wraps something else. */
  cause?: unknown;
};
