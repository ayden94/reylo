# Porg

Import a `.sql` file and get a fully typed query.

```ts
import * as getUser from "./get_user.sql";
```

Porg is a TypeScript 7.1 content mapper that parses the SQL, reconciles it against the real database catalog, and hands the type checker a typed module. A companion runtime loader makes the same import work in Node, nub, and Bun.

Status: pre-alpha. Design notes and experiments live in this repository while the shape settles.

## Repository

Porg is the root of this repository. [`porgapi/`](./porgapi) is PorgAPI, a web framework where the backend is a directory of SQL files, imported with its history on 2026-09-19 so the two projects share one SQL analysis engine (`engine/`, a Cargo workspace member with `porgapi/engine/` and `pg-extension/`). [`AGENTS.md`](./AGENTS.md) describes the layout and the build gates of both; [`DESIGN.md`](./DESIGN.md) §20 records how the two implementations are being unified. [`ROADMAP.md`](./ROADMAP.md) lists the pieces, the order they ship in and the planned work, and [`docs/dialects.md`](./docs/dialects.md) what each SQL dialect can and cannot type. `nub run build` writes the publishable package to `dist/`: the sources compiled to JavaScript with their declarations, and the engine. The repository's own `package.json` points at the TypeScript sources so the tests and labs need no build, and it is private, so only `dist/` can be published.

## The Postgres extension

[`pg-extension/`](./pg-extension) is `pg_porg`, a Postgres extension with its functions in schema `porg` that hosts the same analysis engine inside the server: `SELECT porg.analyze('select …')` returns the whole analysis (parameter and column types, nullability, nested JSON shapes, the row count) in one query, `porg.describe` is Parse/Describe as JSON, and `porg.relation`, `porg.function` and `porg.type` describe the catalog in the same type model, a view's NOT NULL columns included. It is optional. The mapper looks for it once per session; where it finds the same engine build it asks the server for the analysis, where it finds another build it sends each round of catalog reads as one query, and where there is no extension it works as before. A native extension needs the vendor's approval on Neon and Supabase; the [extension's README](./pg-extension/README.md) has the details and the build.

## Bundlers

`porg/vite` is a Vite, Rollup and Rolldown plugin; `porg/esbuild` is an esbuild plugin that `Bun.build` and `Bun.plugin` take as is; `porg/webpack-loader` is a loader for webpack, Rspack and Turbopack. Each turns a `.sql` import into the same module the runtime preload builds, so a bundled app needs no preload.

Each adapter analyzes the statement as the type check does, so a build needs the database, or a `.porg/` that `porg generate` filled where the database could be reached (`.porg/` is the cache they all share, and git ignores it). The module it writes carries the analysis and reads nothing at run time. A dev server rebuilds a `.sql` module when the type check analyzes its statement again.

```sh
$ porg generate          # every .sql file under the current directory, into .porg/
.: postgres at localhost:5432/postgres (DATABASE_URL in .env)
  analyzed  src/get_user.sql    maybeOne  id    12 ms
  cached    src/list_users.sql  many      -      0 ms
2 files in 1 project: 1 analyzed, 1 cached (0.1 s)
```

Next.js takes `porg/webpack-loader` for both of its bundlers, on the server only, so that a client component that imports a `.sql` file fails the build instead of shipping the statement to the browser. `next build` runs `tsc` without `--runExternalCode`, so the mapper cannot run inside it: turn that step off and run the type check before the build ([lab/nextjs](./lab/nextjs) is the whole setup, with Turbopack and webpack, in `next dev` and `next build`):

```ts
// next.config.ts
const nextConfig: NextConfig = {
  typescript: { ignoreBuildErrors: true }, // the build script runs `next typegen && tsc --noEmit --runExternalCode` first
  turbopack: { rules: { "*.sql": { condition: { not: "browser" }, loaders: ["porg/webpack-loader"], as: "*.js" } } },
  webpack(config, { isServer }) {
    if (isServer) config.module.rules.push({ test: /\.sql$/, use: "porg/webpack-loader" });
    return config;
  },
};
```

## Examples

Parameters are written `:name` in every dialect, the form mysql2's `namedPlaceholders` and SQLite use. The engine lexes them out and rewrites them to the database's own placeholder (`$1` for Postgres, `?` for MySQL and SQLite) when the mapper describes the statement; the rewritten text and its binding order land in `.porg/queries.json`, and `run` binds through them, so the parameter object is `{ name: ... }` everywhere. A statement may use its database's own positional form instead (`$1` on Postgres, `?` on MySQL, `?` and `?NNN` on SQLite), and the key is the position's number: `where id = $1` takes `{ 1: ... }`, and a bare `?` is one past the largest number before it, as SQLite numbers it. Mixing the two styles in one statement is an error, and so is a form the database would misread (`$name` or `@name` on SQLite, `$name` on Postgres, `?2` on MySQL); MySQL's `@name` is a user variable and stays one. The engine infers the row count from the statement and the schema on every dialect (a primary-key equality is `maybeOne`, `LIMIT 1` is `maybeOne`, an ungrouped aggregate `one`, a one-row `INSERT … RETURNING` `one`, a data-modifying statement without RETURNING `none`); there is no directive to override it. A parameter nothing in the statement types is `unknown`; `cast(:x as integer)` types it, and the database enforces the cast. A parameter is typed without null unless the statement lets the caller pass one: it tests the parameter for null or coalesces it (Postgres), or writes it whole to a nullable column (Postgres and SQLite).

### Postgres

```sql
select id, name, email, role, tags, created_at, id + 1 as next_id
from porg_users
where id = :id and name = :name
```

```ts
import getUser from "./get_user.sql";
const user = await getUser.run(sql, { id: 1, name: "ada" });
//    ^? { id: number; name: string; email: string | null; role: "admin" | "member"; tags: string[]; created_at: Date; next_id: number } | null
```

`with(client)` binds the query to a client, such as a transaction, and returns a function of the parameters: `const get = getUser.with(tx); await get({ id: 1, name: "ada" })`. A statement that takes no parameters takes no second argument: `await listUsers.run(sql)`. The client is always the caller's.

`run` takes the client object of the driver the project already uses, or a transaction, savepoint or connection taken from it, and Porg imports no driver: pg (Pool, Client, PoolClient), postgres.js and Bun.sql (with `begin`, `savepoint` and `reserve`), PGlite (with `transaction`), Neon (Pool, Client and `neon()`), pg-promise (with `task` and `tx`) and @vercel/postgres; mysql2 and mariadb (promise or callback API) and PlanetScale's serverless driver 1.x (with `transaction`; 2.0 binds no parameters, so it runs only statements without any); better-sqlite3, node:sqlite, bun:sqlite, @libsql/client (with `transaction()`), Cloudflare D1, sqlite3 and sqlite. The query's dialect narrows which of them it accepts. A parameter left out binds NULL on every driver. `run` always returns a promise, which rejects with the driver's own error. On better-sqlite3, node:sqlite and bun:sqlite, which answer synchronously, `runSync` returns the result itself and throws the error, so it works inside their `db.transaction(fn)`, whose function cannot await; any other client is a type error, and a `TypeError` before anything runs:

```ts
// postgres.js and Bun.sql: the transaction and the savepoint are clients.
await sql.begin(async (tx) => {
  const user = await getUser.run(tx, { id: 1, name: "ada" });
  await tx.savepoint((sp) => rename.run(sp, { id: user!.id, name: "bo" }));
});
// better-sqlite3 and bun:sqlite: runSync hands the row back at once, and an error (a constraint,
// or a statement that returns the wrong number of rows) rolls the whole transaction back.
db.transaction(() => {
  const { id } = insertUser.runSync(db, { name: "ada" });
  insertPost.runSync(db, { user_id: id, title: "hello" });
})();
```

Parameter types come from the server (`id: number`, `name: string`); `role` is an enum, `tags` a `text[]`, `email` nullable from the catalog, `next_id` non-null because `id` is and the built-in `+` never returns NULL for non-null operands; `| null` on the row because `id` is the primary key, so the `WHERE` matches at most one row. `created_at` is a `Date` and an `int8` a `string` because that is what every Postgres driver returns by default, and `run` returns the driver's values (see [Dates and 64-bit integers](#dates-and-64-bit-integers)).

Nested JSON keeps its shape. The engine reads the constructors out of the parse tree, has the database type their arguments as ordinary columns, and folds the catalog's `NOT NULL` and the outer joins back in:

```sql
select u.id, u.name,
  coalesce((select jsonb_agg(jsonb_build_object('id', p.id, 'title', p.title, 'body', p.body, 'at', p.published_at)) from porg_posts p where p.user_id = u.id), '[]') as posts,
  to_jsonb(u) as row,
  (select count(*) from porg_posts p where p.user_id = u.id) as post_count
from porg_users u
where u.id = :id
```

```ts
//    ^? { id: number; name: string; posts: { id: number; title: string; body: string | null; at: string | null }[]; row: { id: number; name: string; email: string | null; role: "admin" | "member"; tags: string[]; created_at: string; big: number | null }; post_count: string } | null
```

Inside JSON the values are what the database serialized (`created_at` is a string there, `big` a number), which is what `Json<T>` in the type model spells out.

### SQLite

```sql
select u.id, u.name, p.title, cast(:min as integer) as min_score
from users u left join posts p on p.user_id = u.id
where u.name like :q and u.score >= :min
order by u.id limit :n offset :o
```

```ts
import search from "./search.sql";
const rows = await search.run(db, { q: "a%", min: 1, n: 5, o: 0 });
//    ^? { id: number; name: string; title: string | null; min_score: number | null }[]
```

SQLite has no describe for a parameter or an expression, so the engine carries SQLite itself: the amalgamation, patched with five one-line hooks after name resolution, compiled into the engine's wasm. The mapper reads the schema DDL from `sqlite_schema` through `node:sqlite`, the engine replays it into an in-memory database and prepares the statement there, and SQLite's own resolved tree comes back: every column reference with its table, column, `NOT NULL` and outer-join mark, every expression with SQLite's affinity, every parameter in the context SQLite placed it in. So `:q` is text because SQLite compares it with `u.name`, `:min` is the type of `u.score` (and the cast agrees), `:n` and `:o` are integers because they are the `LIMIT` and `OFFSET`; an `INSERT … VALUES` or `UPDATE … SET` types a placeholder from the column it feeds. What SQLite has no facts for stays a table in the engine: the result types of the built-in functions (`count` is a non-null integer, `coalesce` takes its arguments' type, `length` an integer), the arithmetic rule (`id + 1` is an integer, `score * 2` a real), and the row count from the unique keys: `where id = :id` on an `INTEGER PRIMARY KEY` or a unique index is `| null`, a one-row `INSERT … RETURNING` is one row, `on conflict do nothing` and `insert or ignore` make it `| null`, and a statement without `RETURNING` is `none` (`run` resolves to `void`). Every SQLite form parses, `limit o, n`, `glob`, `insert or replace` included. A parameter no context names (one whose only context is a function call, or a bare `select :x`) is `unknown`, and a `cast(:x as text)` types it.

### MySQL

```sql
select id, name, email, big, n, at, j, flag, id + 1 as next_id
from users
where id = :id and name <> :skip
```

```ts
const user = await getUser.run(conn, { id: 1, skip: "" });
//    ^? { id: number; name: string; email: string | null; big: string; n: string; at: string; j: unknown; flag: number; next_id: string } | null
```

MySQL 8.4's prepare reports column types, NOT NULL and provenance, and infers a type per parameter from context, so no annotation is needed; `bigint` and `datetime` follow the `bigint` and `dates` settings (strings by default; mysql2 decodes a `bigint` as a lossy `number`, which the runtime turns into text) and `decimal` is a string. The engine adds what the server does not say: the row count (`id` is the primary key, so `maybeOne`; an ungrouped aggregate is `one`; an `insert`, `update` or `delete` is `none`, MySQL having no RETURNING), and the shape of a `json_object` / `json_arrayagg` column, typed from its arguments:

```sql
select u.id,
  (select coalesce(json_arrayagg(json_object('id', p.id, 'title', p.title)), json_array())
   from posts p where p.user_id = u.id) as posts
from users u
where u.id = :id
-- { id: number; posts: { id: number; title: string }[] } | null
```

## The project file

Settings live in `.porg.yaml` beside the project's `package.json` (the nearest one at or above the tsconfig; `PORG_CONFIG` names another file), one file per package. A project whose connection is `DATABASE_URL` needs no file; the content mapper itself takes no options in `tsconfig.json`.

```yaml
url: ${DATABASE_URL}          # the default; the secret stays in the environment; the scheme names the dialect
direct_url: ${DIRECT_URL}     # optional: a connection past a pooler, which the type check prefers
env: [.env.local, .env]       # the default; the process environment wins, then the first file that sets a variable
strict: true                  # optional: a parameter nothing in the statement types is an error instead of unknown
types:                        # what the project's driver returns (see below)
  dates: Date
  bigint: string
```

The URL's scheme names the dialect: `postgres://` and `postgresql://`, `mysql://` and `mariadb://`, `file:` and `sqlite:`. A key Porg does not know, or a `${VAR}` nothing sets, is an error on every `.sql` import that names the line and column in `.porg.yaml`; the sections PorgREST and PorgAPI will read (`rest`, `api`) are accepted as they are. A relative SQLite path in the URL resolves against the package directory.

## Dates and 64-bit integers

`run` returns the rows the driver returns: Porg declares their types and converts no value. Drivers disagree on two kinds of value. Every Postgres and MySQL driver decodes a timestamp as a `Date` unless it is set up otherwise, and a 64-bit integer arrives as a string (`pg`, `postgres`, Bun.sql, Neon), a number (mysql2 and the SQLite drivers, lossy above 2^53) or a BigInt (mariadb); the measurements are in [docs/dates.md](./docs/dates.md). The `types` section of `.porg.yaml` says what the project's driver returns, and the rendered type follows it:

```yaml
types:
  dates: Date       # the default on Postgres and MySQL; string or temporal for a driver set up to return those
  bigint: string    # the default on Postgres; number is the default on MySQL and SQLite; bigint for mariadb
```

| Setting | Value | `timestamptz` | `timestamp`, MySQL `datetime` / `timestamp` | `date` | `int8`, MySQL `bigint` |
| --- | --- | --- | --- | --- | --- |
| `dates` | `Date` (default) | `Date` | `Date` | `Date` | |
| `dates` | `string` | `string` | `string` | `string` | |
| `dates` | `temporal` | `Temporal.Instant` | `Temporal.PlainDateTime` | `Temporal.PlainDate` | |
| `bigint` | `string` (Postgres default), `number` (MySQL and SQLite default), `bigint` | | | | `string`, `number` or `bigint` |

- The setting has to match the driver, because Porg does not see the driver's options: `pg` with a `types.setTypeParser` for timestamps, `postgres` with its `types` option, or mysql2 and mariadb with `dateStrings: true` return text and need `dates: string`; mysql2 with `supportBigNumbers: true, bigNumberStrings: true` needs `bigint: string`; mariadb returns a BigInt by default and needs `bigint: bigint`.
- `temporal` makes the generated module reference TypeScript's own `esnext.temporal` lib, which declares the global `Temporal` namespace, so the types resolve whatever `lib` the project sets; the driver builds the values ([lab/repr](./lab/repr) sets up `postgres` to).
- `bigint` gives a `bigint`; `JSON.stringify` has no rule for one, so add a `BigInt.prototype.toJSON` or a replacer.
- Inside a nested JSON shape the database serialized the value already, so the settings do not apply there; SQLite has no date type and its integers follow the driver.
