import type { D1Migration } from "cloudflare:test";

// The binding vitest.config.ts adds for the tests: the migrations, read from migrations/.
declare global {
  namespace Cloudflare {
    interface Env {
      MIGRATIONS: D1Migration[];
    }
  }
}
