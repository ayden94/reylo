import { randomBytes } from 'node:crypto';
import { Client, type ClientConfig } from 'pg';

// Admin URL: a connection that has CREATEDB. Defaults to the local
// homebrew postgres on port 5432 against the maintenance `postgres` db.
// Override with LANDO_TEST_ADMIN_URL for CI or non-default setups.
const DEFAULT_ADMIN_URL = 'postgres://localhost:5432/postgres';

export const adminUrl = (): string => process.env['LANDO_TEST_ADMIN_URL'] ?? DEFAULT_ADMIN_URL;

export interface TestDatabase {
  readonly url: string;
  readonly name: string;
  /** Drop the database. Idempotent; safe to call from `afterAll`. */
  readonly drop: () => Promise<void>;
}

/**
 * Probe whether the admin URL is reachable. Returns `true` if a connection
 * can be opened and a trivial query runs. Used by integration tests to
 * skip themselves cleanly when no PG is available.
 */
export async function isPostgresAvailable(): Promise<boolean> {
  const client = new Client({ connectionString: adminUrl() });
  try {
    await client.connect();
    await client.query('select 1');
    return true;
  } catch {
    return false;
  } finally {
    await client.end().catch(() => {});
  }
}

/**
 * Create a fresh, empty database for the duration of a test or suite.
 * Database names are random, so tests can run in parallel safely.
 *
 * Returns the connection URL plus a `drop()` cleanup. Use `WITH (FORCE)`
 * (PG 13+) so lingering connections don't block teardown.
 */
export async function createTestDatabase(prefix = 'lando_test'): Promise<TestDatabase> {
  const name = `${prefix}_${randomBytes(6).toString('hex')}`;
  await runOnAdmin(async (client) => {
    await client.query(`create database "${name}"`);
  });
  return {
    name,
    url: replaceDatabase(adminUrl(), name),
    drop: () =>
      runOnAdmin(async (client) => {
        await client.query(`drop database if exists "${name}" with (force)`);
      }),
  };
}

async function runOnAdmin<T>(fn: (client: Client) => Promise<T>): Promise<T> {
  const client = new Client({ connectionString: adminUrl() });
  await client.connect();
  try {
    return await fn(client);
  } finally {
    await client.end();
  }
}

function replaceDatabase(url: string, db: string): string {
  // pg accepts URLs without a dbname path; we always force one.
  const u = new URL(url);
  u.pathname = `/${db}`;
  return u.toString();
}

export type { ClientConfig };
