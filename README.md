# Reylo

Import `.sql` files into your TypeScript code with full type inference.

```sql
-- get_author.sql
select u.id, u.name, count(p.id)::int as post_count
from users u
left join posts p on p.user_id = u.id
where u.id = :id
group by u.id
```

```ts
// main.ts
import postgres from "postgres";
import getAuthor from "./get_author.sql";

const db = postgres(process.env.DATABASE_URL!);

const author = await getAuthor.run(db, { id: 1 });
//    ^? { id: number; name: string; post_count: number } | null
```

Reylo is a content mapper for `.sql` files. Content mappers are new in TypeScript 7.1 ([now in beta](https://devblogs.microsoft.com/typescript/announcing-typescript-7-1-beta/)): an official way to plug other file types into TypeScript's compilation. Reylo reads each statement, asks your database what it takes and returns, and hands TypeScript a typed module.

- 💪 **The full power of SQL.** Any statement your database runs, in its own dialect.
- 🌵 **DRY.** The types come from the SQL itself: no hand-written row types, no schema file to keep in sync.
- 🎯 **Checked against your database.** A misspelled column, a wrong parameter or a dropped table is a type error, in `tsc` and in your editor.
- 🤖 **Agent-friendly.** Each fact lives in one place, so an agent cannot leave a stale type behind, and the type check finds every caller it missed.
- 🧩 **Postgres, MySQL and SQLite.** Bring your own driver: `pg`, postgres.js, mysql2, better-sqlite3, Bun, Cloudflare D1 and [more](#supported-drivers).
- ⚡ **No codegen.** The type checker asks Reylo directly: no generated files, no watcher.

## Content mappers

Before TypeScript 7.1, a `.sql` import could only be typed by a declaration written ahead of time: a wildcard `declare module "*.sql"`, which gives every query the same loose type, or a `.d.ts` file that a code generator writes beside each query and has to keep fresh.

A content mapper is a package that `tsconfig.json` names for a file extension:

```jsonc
"contentMappers": [{ "package": "reylo", "extensions": [".sql"] }]
```

Whenever the type checker meets an import of a `.sql` file, in `tsc` or in your editor, it hands the file to Reylo and checks your code against the module Reylo returns. An error Reylo finds shows at its line and column inside the `.sql` file. It is TypeScript's own mechanism, so CI and your editor see the same types.

To type a statement, Reylo has your development database prepare it, without running it, and reads the catalog. SQLite statements are prepared in Reylo's own copy of SQLite, against your database file's schema. From the answer, Reylo learns:

- each parameter's type
- each column's type
- which columns can be `null`, from `NOT NULL` and outer joins
- how many rows the statement can return, from keys, aggregates, `limit 1` and `returning`

Reylo keeps each answer in `.reylo/`, keyed by a fingerprint of the schema, so it analyzes a statement again only for a schema it has not seen. After a migration, a statement that names a dropped column fails in its `.sql` file, and code that reads a column the statement no longer returns fails in TypeScript.

At run time, the same import becomes a module that runs the statement through your own driver. Node's `--import`, a Bun or nub preload, or a bundler plugin builds it.

## Setup

Reylo needs:

- **TypeScript 7.1 beta or later**, the first release with content mappers.
- **Node 24 or later, Bun or nub.**
- **A running development database**: Postgres, MySQL 8.0.22 or later, or a SQLite file. Reylo reads every type from it.

```sh
npm install reylo
npm install -D @reylosql/postgres  # or @reylosql/mysql, @reylosql/sqlite
npm install -D typescript@beta
```

`reylo` is the content mapper, the run time, the bundler plugins, the preload and the CLI. The engine that reads your database's statements is a package of its own, one per database, at Reylo's version: install the one for each database the project uses. A development dependency does, since a built module carries its analysis; a program run under the preload analyzes as it imports, and needs it installed. Without it, a `.sql` import reports what to install: `postgres statements need @reylosql/postgres: npm install -D @reylosql/postgres`.

Name Reylo in `tsconfig.json`, and type-check with `--runExternalCode`, the flag that lets `tsc` run content mappers:

```jsonc
// tsconfig.json
{
  "compilerOptions": { /* ... */ },
  "contentMappers": [
    { "package": "reylo", "extensions": [".sql"] }
  ]
}
```

```jsonc
// package.json
{
  "scripts": {
    "typecheck": "tsc --noEmit --runExternalCode"
  }
}
```

Set `DATABASE_URL` to your development database, in the environment or in `.env`. The scheme names the dialect:

```sh
# .env: postgres://…, mysql://…, or file:./app.db for SQLite
DATABASE_URL=postgres://localhost:5432/app
```

Load Reylo when your code runs, so that a `.sql` import works at run time too. On Node:

```sh
node --env-file=.env --import reylo/register main.ts
```

| Runtime or bundler | Setup |
| --- | --- |
| Bun | `preload = ["reylo/register"]` in `bunfig.toml` |
| nub | `"preload": ["reylo/register"]` in `nub.jsonc` |
| Vite, Vitest | `import reylo from "reylo/vite"`, then `reylo()` in `plugins` |
| esbuild, `Bun.build` | `import reylo from "reylo/esbuild"`, then `reylo()` in `plugins` |
| webpack, Rspack | `{ test: /\.sql$/, use: "reylo/webpack-loader" }` in `module.rules` |
| Next.js | `reylo/webpack-loader`, on the server, for Turbopack and webpack ([setup](https://reylo.dev/#setup-nextjs)) |
| Jest | `transform: { "\\.sql$": "reylo/jest" }` ([setup](https://reylo.dev/#setup-jest)) |

In VS Code or Cursor, install the **TypeScript 7** extension (`TypeScriptTeam.native-preview`) and point it at your project's TypeScript, since the extension bundles TypeScript 7.0:

```jsonc
// .vscode/settings.json
{
  "js/ts.experimental.useTsgo": true,
  "js/ts.tsdk.path": "./node_modules/typescript"
}
```

Then trust the workspace, and click **Allow** when the extension asks to use the workspace's TypeScript. The editor does not see your shell's environment, so keep `DATABASE_URL` in `.env`. Zed takes a few more settings: see [Editors](https://reylo.dev/#editors).

## Usage

Write one statement per `.sql` file, with a `:name` for each parameter, and import it. `run` takes a client from your own driver, here `db`, and the parameters.

<details>
<summary>The schema behind the examples</summary>

```sql
create type user_role as enum ('admin', 'member');

create table users (
  id serial primary key,
  name text not null,
  email text unique,
  role user_role not null default 'member',
  tags text[] not null default '{}',
  created_at timestamptz not null default now()
);

create table posts (
  id serial primary key,
  user_id integer not null references users (id),
  title text not null,
  body text,
  published_at timestamptz
);
```

</details>

```sql
-- get_user.sql
select id, name, email from users where id = :id
```

```ts
import getUser from "./get_user.sql";

const user = await getUser.run(db, { id: 1 });
//    ^? { id: number; name: string; email: string | null } | null
```

`id` is the primary key, so the result is one row or `null`, and `email` is a nullable column. An insert with `returning` resolves to the row it wrote:

```sql
-- create_user.sql
insert into users (name, email) values (:name, :email)
returning id
```

```ts
import createUser from "./create_user.sql";

const { id } = await createUser.run(db, { name: "Ada", email: null });
//      ^? number
```

A statement that can match many rows resolves to an array:

```sql
-- list_users.sql
select id, name, role, created_at from users
where role = :role
order by created_at desc
```

```ts
import listUsers from "./list_users.sql";

const admins = await listUsers.run(db, { role: "admin" });
//    ^? { id: number; name: string; role: "admin" | "member"; created_at: string }[]
```

What `run` resolves to follows from how many rows Reylo proves the statement can return, and `run` checks the count when it runs:

| The statement | `run` resolves to |
| --- | --- |
| A match on a primary or unique key, or `limit 1` | `Row \| null` |
| An aggregate without `group by`, or a one-row `insert … returning` | `Row` |
| An `insert`, `update` or `delete` without `returning` | `void` |
| Anything else | `Row[]` |

### Errors

A mistake in a `.sql` file is a type error at its line and column:

```text
$ tsc --noEmit --runExternalCode
get_user.sql:1:12 - error reylo6: column "nmae" does not exist (42703)

1 select id, nmae, email from users where id = :id
             ~~~~

Found 1 error in get_user.sql:1
```

A wrong argument is an error where you call the query:

```text
main.ts:4:38 - error TS2322: Type 'string' is not assignable to type 'number'.

4 const user = await getUser.run(db, { id: "1" });
                                       ~~
```

### Nested JSON

JSON built in the statement keeps its shape:

```sql
-- user_with_posts.sql
select u.id, u.name,
  coalesce((
    select jsonb_agg(jsonb_build_object(
      'id', p.id,
      'title', p.title,
      'at', p.published_at
    ))
    from posts p where p.user_id = u.id
  ), '[]') as posts
from users u
where u.id = :id
```

```ts
import userWithPosts from "./user_with_posts.sql";

const user = await userWithPosts.run(db, { id: 1 });
// {
//   id: number;
//   name: string;
//   posts: { id: number; title: string; at: string | null }[];
// } | null
```

Inside JSON, a value is what the database wrote, so the timestamp `at` is a string there.

### JSON parameters

A JSON parameter carries a [JSON Schema](https://json-schema.org) in its cast. The schema types the parameter, and Reylo checks every value against it before the statement runs, so a list or a batch of rows is one parameter:

```sql
-- add_users.sql
with input as (
  select :names::jsonb('{ "type": "array", "items": { "type": "string" } }') as names
)
insert into users (name)
select jsonb_array_elements_text(input.names) from input
```

```ts
import addUsers from "./add_users.sql";

await addUsers.run(db, { names: ["Grace", "Edsger"] });
//                       ^? names: string[]
```

### Transactions

`run` takes a transaction, a savepoint or a connection from your driver as its client:

```ts
import createUser from "./create_user.sql";
import renameUser from "./rename_user.sql"; // update users set name = :name where id = :id

await db.begin(async (tx) => {
  const { id } = await createUser.run(tx, { name: "Barbara", email: null });
  await renameUser.run(tx, { id, name: "Barbara Liskov" });
});
```

On better-sqlite3, `node:sqlite` and `bun:sqlite`, `runSync` runs a statement synchronously and returns its result, for code that cannot await, such as the function that better-sqlite3's `db.transaction` takes.

### The imported query

Besides `run`, an imported query carries its statement, as Reylo sends it to the database (`sql`) and as written (`raw`), both typed as string literals, and its schemas, and the module exports its types:

```ts
import getUser, { type Params, type Row } from "./get_user.sql";

const get = getUser.with(db);  // bound to one client: await get({ id: 1 })
getUser.sql;                   // "select id, name, email from users where id = $1\n" (? on MySQL and SQLite)
getUser.raw;                   // "select id, name, email from users where id = :id\n"
getUser.params;                // ["id"]
getUser.cardinality;           // "maybeOne"
getUser.inputSchema;           // the parameters, as JSON Schema
getUser.outputSchema;          // the result, as JSON Schema
```

## Supported drivers

Reylo is bring-your-own-driver: `run` takes a client from any of these, or a transaction, savepoint or connection taken from one.

| Database | Drivers |
| --- | --- |
| Postgres | `pg`, postgres.js, Bun.sql, PGlite, Neon, pg-promise, `@vercel/postgres` |
| MySQL | mysql2, mariadb, PlanetScale 1.x, Bun.sql |
| SQLite | better-sqlite3, `node:sqlite`, `bun:sqlite`, libSQL, Cloudflare D1, sqlite3, sqlite, Bun.sql |

The type check reads the schema through `pg` or postgres.js on Postgres and through mysql2 on MySQL, so an app on another driver adds one of those as a dev dependency.

Every driver gives Reylo's queries the same values, however your app set it up: Reylo decodes each value of its own queries itself, on your client, and leaves your other queries alone. A 64-bit integer is a `bigint`, a decimal its exact text, an instant (`timestamptz`) the ISO 8601 text of the instant in UTC (`"2026-10-20T08:00:00.123456Z"`, which `Temporal.Instant.from` reads), a date or a time without a zone the text the database wrote (`"2026-10-20"`, `"2026-10-20T10:00:00.123"`), a truth value a `boolean` (MySQL's `BOOLEAN`, which is a `TINYINT(1)`, and a comparison too), JSON is parsed and binary data is a `Uint8Array`; on SQLite an integer past 2^53 is an error naming its column, never a rounded number.

## Configuration

Reylo needs no configuration when the database is `DATABASE_URL`. Settings go in `.reylo.yaml`, beside `package.json`:

```yaml
url: ${DATABASE_URL}  # the default
strict: true          # a parameter nothing in the statement types is an error
```

## Documentation

The [full documentation](https://reylo.dev) covers every editor, runtime and bundler, parameters and casts, each setting, the data types of each dialect, and examples by task, from paging through rows to inserting related rows. The examples above use Postgres; the documentation shows every example in Postgres, MySQL and SQLite.
