# Porg

Import a `.sql` file and get a fully typed query.

```ts
import * as getUser from "./get_user.sql";
```

Porg is a TypeScript 7.1 content mapper that parses the SQL, reconciles it against the real database catalog, and hands the type checker a typed module. A companion runtime loader makes the same import work in Node, nub, and Bun.

Status: pre-alpha. Design notes and experiments live in this repository while the shape settles.

## Repository

Porg is the root of this repository. [`porgapi/`](./porgapi) is PorgAPI, a web framework where the backend is a directory of SQL files, imported with its history on 2026-09-19 so the two projects share one SQL analysis engine (`engine/`, a Cargo workspace member with `porgapi/engine/` and `pg-extension/`). [`AGENTS.md`](./AGENTS.md) describes the layout and the build gates of both; [`DESIGN.md`](./DESIGN.md) §20 records how the two implementations are being unified. [`ROADMAP.md`](./ROADMAP.md) lists the pieces, the order they ship in and the planned work, and [`docs/dialects.md`](./docs/dialects.md) what each SQL dialect can and cannot type.

## The Postgres extension

[`pg-extension/`](./pg-extension) is `pg_porg`, a Postgres extension with its functions in schema `porg` that hosts the same analysis engine inside the server: `SELECT porg.analyze('select …')` returns the whole analysis (parameter and column types, nullability, nested JSON shapes, the row count) in one query, `porg.describe` is Parse/Describe as JSON, and `porg.relation`, `porg.function` and `porg.type` describe the catalog in the same type model, a view's NOT NULL columns included. It is optional. The mapper looks for it once per session; where it finds the same engine build it asks the server for the analysis, where it finds another build it sends each round of catalog reads as one query, and where there is no extension it works as before. A native extension needs the vendor's approval on Neon and Supabase; the [extension's README](./pg-extension/README.md) has the details and the build.

## Bundlers

`porg/vite` is a Vite, Rollup and Rolldown plugin; `porg/esbuild` is an esbuild plugin that `Bun.build` and `Bun.plugin` take as is; `porg/webpack-loader` is a loader for webpack, Rspack and Turbopack. Each turns a `.sql` import into the same module the runtime preload builds, so a bundled app needs no preload.

## Examples

Parameters are written `:name` in every dialect (`$name` and `@name` work too). The engine lexes them out and rewrites to the database's own placeholder (`$1` for Postgres, `?` for MySQL, `:name` stays for SQLite) when the mapper describes the statement; the rewritten text and its binding order land in `.porg/queries.json`, and `execute` binds through them, so the parameter object is `{ name: ... }` everywhere. Positional `$1` / `?` are accepted too; mixing the two styles in one statement is an error. The engine infers the row count from the statement and the schema on every dialect (a primary-key equality is `maybeOne`, an ungrouped aggregate `one`, a one-row `INSERT … RETURNING` `one`, a data-modifying statement without RETURNING `none`); `-- @returns one | maybeOne | many | none` overrides it. A parameter nothing in the statement types is `unknown`; `cast(:x as integer)` types it, and the database enforces the cast. A parameter is typed without null unless the statement lets the caller pass one: it tests the parameter for null or coalesces it (Postgres), or writes it whole to a nullable column (Postgres and SQLite).

### Postgres

```sql
select id, name, email, role, tags, created_at, id + 1 as next_id
from lando_users
where id = :id and name = :name
```

```ts
import getUser from "./get_user.sql";
const user = await getUser.execute(sql, { id: 1, name: "ada" });
//    ^? { id: number; name: string; email: string | null; role: "admin" | "member"; tags: string[]; created_at: string; next_id: number } | null
```

Parameter types come from the server (`id: number`, `name: string`); `role` is an enum, `tags` a `text[]`, `email` nullable from the catalog, `next_id` non-null because `id` is and the built-in `+` never returns NULL for non-null operands; `| null` on the row because `id` is the primary key, so the `WHERE` matches at most one row. `created_at` is an ISO 8601 `string` and an `int8` a `string` because those are the default representations (see [Dates and 64-bit integers](#dates-and-64-bit-integers)).

Nested JSON keeps its shape. The engine reads the constructors out of the parse tree, has the database type their arguments as ordinary columns, and folds the catalog's `NOT NULL` and the outer joins back in:

```sql
select u.id, u.name,
  coalesce((select jsonb_agg(jsonb_build_object('id', p.id, 'title', p.title, 'body', p.body, 'at', p.published_at)) from lando_posts p where p.user_id = u.id), '[]') as posts,
  to_jsonb(u) as row,
  (select count(*) from lando_posts p where p.user_id = u.id) as post_count
from lando_users u
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
const rows = await search.execute(db, { q: "a%", min: 1, n: 5, o: 0 });
//    ^? { id: number; name: string; title: string | null; min_score: number | null }[]
```

SQLite has no describe for a parameter or an expression, so the engine carries SQLite itself: the amalgamation, patched with five one-line hooks after name resolution, compiled into the engine's wasm. The mapper reads the schema DDL from `sqlite_schema` through `node:sqlite`, the engine replays it into an in-memory database and prepares the statement there, and SQLite's own resolved tree comes back: every column reference with its table, column, `NOT NULL` and outer-join mark, every expression with SQLite's affinity, every parameter in the context SQLite placed it in. So `:q` is text because SQLite compares it with `u.name`, `:min` is the type of `u.score` (and the cast agrees), `:n` and `:o` are integers because they are the `LIMIT` and `OFFSET`; an `INSERT … VALUES` or `UPDATE … SET` types a placeholder from the column it feeds. What SQLite has no facts for stays a table in the engine: the result types of the built-in functions (`count` is a non-null integer, `coalesce` takes its arguments' type, `length` an integer), the arithmetic rule (`id + 1` is an integer, `score * 2` a real), and the row count from the unique keys: `where id = :id` on an `INTEGER PRIMARY KEY` or a unique index is `| null`, a one-row `INSERT … RETURNING` is one row, `on conflict do nothing` and `insert or ignore` make it `| null`, and a statement without `RETURNING` is `none` (`execute` resolves to `void`). Every SQLite form parses, `limit o, n`, `glob`, `insert or replace` included. A parameter no context names (one whose only context is a function call, or a bare `select :x`) is `unknown`, and a `cast(:x as text)` types it.

### MySQL

```sql
select id, name, email, big, n, at, j, flag, id + 1 as next_id
from users
where id = :id and name <> :skip
```

```ts
const user = await getUser.execute(conn, { id: 1, skip: "" });
//    ^? { id: number; name: string; email: string | null; big: string; n: string; at: string; j: unknown; flag: number; next_id: string } | null
```

MySQL 8.4's prepare reports column types, NOT NULL and provenance, and infers a type per parameter from context, so no annotation is needed; `bigint` and `datetime` follow the `bigint` and `dates` options (strings by default; mysql2 decodes a `bigint` as a lossy `number`, which the runtime turns into text) and `decimal` is a string. The engine adds what the server does not say: the row count (`id` is the primary key, so `maybeOne`; an ungrouped aggregate is `one`; an `insert`, `update` or `delete` is `none`, MySQL having no RETURNING), and the shape of a `json_object` / `json_arrayagg` column, typed from its arguments:

```sql
select u.id,
  (select coalesce(json_arrayagg(json_object('id', p.id, 'title', p.title)), json_array())
   from posts p where p.user_id = u.id) as posts
from users u
where u.id = :id
-- { id: number; posts: { id: number; title: string }[] } | null
```

## Dates and 64-bit integers

Every Postgres and MySQL driver decodes a timestamp as a `Date`, and they disagree on 64-bit integers: `pg`, `postgres` and Bun.sql return a string, mysql2 and the SQLite drivers a number that is lossy above 2^53, mariadb a BigInt (the measurements are in [docs/dates.md](./docs/dates.md)). Two mapper options name one representation; the rendered type follows it, and `execute` produces it whatever the driver returned:

```jsonc
"contentMappers": [{ "package": "porg", "extensions": [".sql"], "options": { "dates": "string", "bigint": "string" } }]
```

| Option | Value | `timestamptz` | `timestamp`, MySQL `datetime` / `timestamp` | `date` | `int8`, MySQL `bigint` |
| --- | --- | --- | --- | --- | --- |
| `dates` | `"string"` (default) | ISO 8601 with `Z`: `2026-01-02T03:04:05.678Z` | the stored wall clock, no zone: `2026-01-02T03:04:05.678` | `2026-01-02` | |
| `dates` | `"Date"` | the driver's `Date` | the driver's `Date` | the driver's `Date` | |
| `dates` | `"temporal"` | `Temporal.Instant` | `Temporal.PlainDateTime` | `Temporal.PlainDate` | |
| `bigint` | `"string"` (default), `"number"`, `"bigint"` | | | | `string`, `number` or `bigint` |

- The default is text on both because it is never lossy, `JSON.stringify(row)` round-trips, and the wire type then equals the JSON twin that `Json<T>` describes.
- A zone-less `timestamp` or `date` reaches Porg as a `Date` the driver built from wall-clock fields (local time in `pg`, `postgres`, PGlite, Bun.sql, mysql2 and mariadb; a `date` is UTC midnight in `postgres`, PGlite and Bun.sql), and Porg reads the fields back in that frame, so the text is the stored wall clock. A driver configured to return the text form (`pg` `types.setTypeParser`, `postgres` `types`, PGlite `parsers`, mysql2 and mariadb `dateStrings`) passes through under `"string"` and is parsed under `"temporal"`.
- `"temporal"` makes the generated module import `porg/temporal`, which declares the global `Temporal` namespace from `temporal-spec`. At runtime Node 26 has a global `Temporal`; Bun does not, so assign a polyfill to `globalThis.Temporal` or `execute` throws with that instruction.
- `"bigint"` gives a `bigint`; `JSON.stringify` has no rule for one, so add a `BigInt.prototype.toJSON` or a replacer. A value above 2^53 that the driver already decoded as a number is lost before Porg sees it: mysql2 takes `supportBigNumbers: true, bigNumberStrings: true`, better-sqlite3 and bun:sqlite `safeIntegers`, node:sqlite `readBigInts`.
- Inside a nested JSON shape the database serialized the value already, so the options do not apply there; SQLite has no date type and its integers follow the driver, so neither option changes a SQLite type.
- The runtime reads the column classes from the mapper's cache (`.porg/queries.json`); before the first `tsc --runExternalCode` run it returns the driver's values unchanged.
