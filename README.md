# Lando

Import a `.sql` file and get a fully typed query.

```ts
import * as getUser from "./get_user.sql";
```

Lando is a TypeScript 7.1 content mapper that parses the SQL, reconciles it against the real database catalog, and hands the type checker a typed module. A companion runtime loader makes the same import work in Node, nub, and Bun.

Status: pre-alpha. Design notes and experiments live in this repository while the shape settles.

## Examples

Parameters are written `:name` in every dialect (`$name` and `@name` work too). The engine lexes them out and rewrites to the database's own placeholder (`$1` for Postgres, `?` for MySQL, `:name` stays for SQLite) when the mapper describes the statement; the rewritten text and its binding order land in `.lando/queries.json`, and `execute` binds through them, so the parameter object is `{ name: ... }` everywhere. Positional `$1` / `?` are accepted too; mixing the two styles in one statement is an error. On Postgres the engine infers the row count from the statement and the catalog (a primary-key equality is `maybeOne`, an ungrouped aggregate `one`, a one-row `INSERT … RETURNING` `one`); `-- @returns one | maybeOne | many | none` overrides it, and `-- @param name type` supplies a parameter type where the database reports none.

### Postgres

```sql
select id, name, email, role, tags, created_at, id + 1 as next_id
from lando_users
where id = :id and name = :name
```

```ts
import getUser from "./get_user.sql";
const user = await getUser.execute(sql, { id: 1, name: "ada" });
//    ^? { id: number; name: string; email: string | null; role: "admin" | "member"; tags: string[]; created_at: Date; next_id: number } | null
```

Parameter types come from the server (`id: number`, `name: string`); `role` is an enum, `tags` a `text[]`, `email` nullable from the catalog, `next_id` non-null because `id` is and the built-in `+` never returns NULL for non-null operands; `| null` on the row because `id` is the primary key, so the `WHERE` matches at most one row. `int8` arrives as `string` because that is what `pg` and `postgres` decode.

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
