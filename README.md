# Lando

Import a `.sql` file and get a fully typed query.

```ts
import * as getUser from "./get_user.sql";
```

Lando is a TypeScript 7.1 content mapper that parses the SQL, reconciles it against the real database catalog, and hands the type checker a typed module. A companion runtime loader makes the same import work in Node, nub, and Bun.

Status: pre-alpha. Design notes and experiments live in this repository while the shape settles.

## Examples

Parameters are written `:name` in every dialect. Lando lexes them out and rewrites to the engine's own placeholder (`$1` for Postgres, `?` for MySQL, `:name` stays for SQLite) both when the mapper describes the statement and when `execute` runs it, so the parameter object is `{ name: ... }` everywhere. Positional `$1` / `?` are accepted too; mixing the two styles in one statement is an error. Comment directives carry what the engine cannot infer: `-- @returns one | maybeOne | many` for cardinality, `-- @param name type` for a parameter type the engine does not report.

### Postgres

```sql
-- @returns maybeOne
select id, name, email, role, tags, created_at, id + 1 as next_id
from lando_users
where id = :id and name = :name
```

```ts
import getUser from "./get_user.sql";
const user = await getUser.execute(sql, { id: 1, name: "ada" });
//    ^? { id: number; name: string; email: string | null; role: "admin" | "member"; tags: string[]; created_at: Date; next_id: number | null } | null
```

Parameter types come from the server (`id: number`, `name: string`); `role` is an enum, `tags` a `text[]`, `email` nullable from the catalog, `next_id` an expression and therefore nullable until the engine's expression rules land; `int8` would arrive as `string` because that is what `pg` and `postgres` decode.

### SQLite

```sql
-- @returns maybeOne
-- @param id number
select id, name, email, score, avatar, created_at, id + 1 as next_id
from users
where id = :id
```

```ts
const user = await getUser.execute(db, { id: 1 });
//    ^? { id: number; name: string; email: string | null; score: number | null; avatar: Uint8Array | null; created_at: string; next_id: unknown } | null
```

SQLite types plain table columns from their declared type and NOT NULL, leaves expressions `unknown`, and never types parameters, so the `-- @param` line is required (the mapper reports the placeholder when it is missing).

### MySQL

```sql
-- @returns maybeOne
select id, name, email, big, n, at, j, flag, id + 1 as next_id
from users
where id = :id and name <> :skip
```

```ts
const user = await getUser.execute(conn, { id: 1, skip: "" });
//    ^? { id: number; name: string; email: string | null; big: number; n: string; at: Date; j: unknown; flag: number; next_id: number } | null
```

MySQL 8.4's prepare reports column types, NOT NULL and provenance, and infers a type per parameter from context, so no annotation is needed here; `bigint` is a lossy `number` under mysql2's defaults and `decimal` a string.
