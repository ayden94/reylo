// Each test file gets a fresh D1 database: the migrations make its tables and rows.
import { applyD1Migrations, env } from "cloudflare:test";

await applyD1Migrations(env.DB, env.MIGRATIONS);
