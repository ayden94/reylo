import { afterAll, beforeAll, describe, expect, it } from 'vitest';
import { Client } from 'pg';
import { createTestDatabase, isPostgresAvailable, type TestDatabase } from '#test/postgres';

const pgAvailable = await isPostgresAvailable();

describe.skipIf(!pgAvailable)('local postgres harness', () => {
  let db: TestDatabase;

  beforeAll(async () => {
    db = await createTestDatabase();
  });

  afterAll(async () => {
    await db.drop();
  });

  it('creates an isolated database and accepts ddl', async () => {
    const client = new Client({ connectionString: db.url });
    await client.connect();
    try {
      await client.query('create table thing (id serial primary key, name text not null)');
      await client.query("insert into thing (name) values ('a'), ('b')");
      const { rows } = await client.query<{ name: string }>('select name from thing order by id');
      expect(rows.map((r) => r.name)).toEqual(['a', 'b']);
    } finally {
      await client.end();
    }
  });
});

describe.skipIf(pgAvailable)('local postgres harness (skipped — no pg)', () => {
  it.skip('requires a local postgres on $LANDO_TEST_ADMIN_URL or localhost:5432', () => {});
});
