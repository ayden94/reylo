// `lando.config.ts` — the user-facing config type. Lives here, re-
// exported from the package root (`import type { LandoConfig } from
// 'lando'`).
//
// The config has two readers: the CLI (Phases 6, 7) for init / generate
// / typecheck, and the runtime ORM (Phase 9) for query execution.
// Anything that's used by *only one* of those should live elsewhere
// (CLI flags in `src/cli/`, runtime options on the `Lando(...)` call).

import type { Adapter } from '#schema';
import type { PolymorphismOverlay } from '#prisma';

export type LandoConfig = {
  source: SchemaSource;
  adapter: Adapter;
  polymorphism?: PolymorphismOverlay;
  /**
   * When to run introspection in the BYO-DB flow. `'build'` (default)
   * caches the result into `.lando/schema.ts`; `'runtime'` re-
   * introspects on every process start. Whitepaper §"Schema source:
   * BYO database URL" recommends `'build'` for production deploys.
   */
  introspectAt?: 'build' | 'runtime';
  /**
   * Runtime input validation toggle. `'on'` validates every `.run()`
   * payload against the schema; `'off'` (default) trusts the DB.
   * Whitepaper Open Question §3 / TODO Decisions §5.
   */
  validate?: { input: 'on' | 'off' };
};

export type SchemaSource =
  | { kind: 'prisma'; path: string }
  | { kind: 'introspection'; url: string };
