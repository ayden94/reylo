// Adapter contract. The adapter is the seam between the database flavor
// and everything else. The same value flows to the CLI (where it's used
// for introspection) and the runtime ORM (where it's used to execute
// SQL); whitepaper §"Schema source" specifies this. v0.1 ships exactly
// one implementation, `pg()` in `src/pg/`.
//
// `Adapter` is intentionally small. The full method surface (the SQL
// AST it accepts, the row shape it returns) belongs to Phase 5
// (introspection) and Phase 10 (driver). Here we declare just enough
// for the config loader to type-check the user's `lando.config.ts`
// without dragging in the rest of the package.

export type AdapterCapabilities = {
  /** Engine supports CHECK constraints on tables. PG yes, SQLite yes. */
  readonly checkConstraints: boolean;
  /** Engine supports expression indexes (`CREATE INDEX … ON t (lower(name))`). */
  readonly expressionIndexes: boolean;
  /** Engine supports `LATERAL` joins — required for our nested-fetch lowering. */
  readonly lateral: boolean;
  /** Engine supports `FOR UPDATE SKIP LOCKED` — required for job-queue patterns. */
  readonly skipLocked: boolean;
};

/**
 * The adapter as seen from the config layer. Phase 5 / 10 will refine
 * this with `introspect()` and `connect()` methods; consumers in those
 * phases narrow the value via the `kind` discriminator.
 */
export type Adapter = {
  /** Stable engine identifier — `'pg'`, `'sqlite'`, etc. Always lower-case. */
  readonly kind: string;
  readonly capabilities: AdapterCapabilities;
};
