// `lando/pg` — the unified Postgres adapter. The same value the user
// constructs in `lando.config.ts` flows to the CLI (Phase 5
// introspection) and the runtime ORM (Phase 10 SQL execution).
// Whitepaper §"Schema source" specifies this single-adapter shape.
//
// Phase 3 lands the factory and the capability flags; the introspection
// + execution methods get filled in by their respective phases.

import type { Adapter, AdapterCapabilities } from '#schema';

export type PgAdapterOptions = {
  /**
   * Connection URL. When omitted, the URL is taken from `source.url`
   * (BYO-DB) or the runtime `Lando({ url })` call. v0.1 prefers
   * runtime resolution so secrets don't end up in `lando.config.ts`.
   */
  url?: string;
};

export type PgAdapter = Adapter & {
  readonly kind: 'pg';
  readonly url: string | undefined;
};

const PG_CAPABILITIES: AdapterCapabilities = {
  checkConstraints: true,
  expressionIndexes: true,
  lateral: true,
  skipLocked: true,
};

/**
 * Constructs a Postgres adapter for use in `lando.config.ts`.
 *
 * ```ts
 * import { pg } from 'lando/pg';
 * export default {
 *   source: { kind: 'introspection', url: process.env.DATABASE_URL! },
 *   adapter: pg(),
 * } satisfies LandoConfig;
 * ```
 */
export function pg(opts: PgAdapterOptions = {}): PgAdapter {
  return {
    kind: 'pg',
    capabilities: PG_CAPABILITIES,
    url: opts.url,
  };
}
