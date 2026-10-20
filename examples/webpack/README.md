# Reylo example: webpack

A small Node HTTP server that imports `.sql` files as typed queries and runs them on MySQL through mysql2, bundled by webpack 5 into one ES module, `dist/server.mjs`, that runs with `node` and no preload. It is the setup of [webpack](https://reylo.dev/#setup-webpack) in the docs.

## Run it

```sh
nub run build                 # at the repository root, once: writes dist/, which this example links
cd examples/webpack
nub install                   # or pnpm install (npm does not take link: dependencies)
cp .env.example .env          # DATABASE_URL=mysql://root@127.0.0.1:3306/reylo_example
nub run setup                 # creates the database unless it exists, then users and posts: "created users and posts"
nub run typecheck             # tsc --noEmit --runExternalCode: no output, exit 0
nub run dev                   # the sources under the preload, restarted on every change, .sql files included
nub run build                 # webpack: "webpack 5.112.0 compiled successfully", writes dist/server.mjs
nub run start                 # node dist/server.mjs, on http://localhost:3000 (PORT changes it)
```

The server answers three routes, the same in `dev` and from the bundle:

```text
$ curl localhost:3000/posts
[{"id":1,"title":"Hello","body":"first post","author":"ada"},{"id":2,"title":"Again","body":null,"author":"ada"}]
$ curl localhost:3000/users/1
{"id":1,"name":"ada","email":"ada@example.com","role":"admin","created_at":"2026-10-20T14:44:58"}
$ curl localhost:3000/users/99
{"error":"no such user"}
$ curl -X POST -d '{"user_id":2,"title":"From bob"}' localhost:3000/posts
{"id":"3"}
```

`/users/99` answers 404, and the `POST` 201. Each `POST` adds a post; `nub run setup` starts over. `DATABASE_URL` in the environment wins over `.env`. `dist/server.mjs` carries mysql2 and Reylo's run time, so it runs from any directory, without `node_modules`.

Misspell a column in `queries/get_user.sql` and run `nub run typecheck` again: the error names the file, such as `queries/get_user.sql(1,1): error reylo6: Unknown column 'nmae' in 'field list' (ER_BAD_FIELD_ERROR)`. MySQL reports no position for a prepare error, so the error covers the whole statement. `nub run build` fails on it too, with `queries/get_user.sql:1:1: Unknown column 'nmae' in 'field list' (ER_BAD_FIELD_ERROR)`.

## How Reylo is wired in

- `package.json`: `reylo` is a dependency, because each `.sql` module imports its run time, which webpack bundles into `dist/server.mjs`. The MySQL engine `@reylosql/mysql` and `typescript` 7.1 are dev dependencies, used by the type check and by the build. `mysql2` is the driver the app runs the queries with, and the type check and the build read the schema through it too.
- `webpack.config.mjs`: the rule `{ test: /\.sql$/, use: "reylo/webpack-loader" }` turns each `.sql` import into a module that runs the statement. The loader analyzes the statement at build time, against the database `DATABASE_URL` names or from `.reylo/`, so the bundle needs neither the database's schema nor a preload when it runs. The rest of the file is webpack's own setup for a Node server: `target: "node24"`, `output.module` for an ES module, and swc-loader for TypeScript.
- `package.json` scripts: `typecheck` is `tsc --noEmit --runExternalCode`, the flag that lets `tsc` run the content mapper. `build` is `webpack`, which needs nothing from Reylo but the rule. `start` is `node --env-file-if-exists=.env dist/server.mjs`, with no preload. `dev` runs the sources with `node --watch --env-file-if-exists=.env --import reylo/register src/server.ts`: Node strips the types, `--import reylo/register` turns each `.sql` import into a module as Node loads it, and `--watch` restarts on a change to a `.sql` file too.
- `tsconfig.json`: `contentMappers` names `reylo` for `.sql`, which types each import from the database. `include` is `src`, `queries` and `setup.ts`, so every query is checked, imported or not, and `setup.sql`, a script rather than one query, stays out. `verbatimModuleSyntax` and `erasableSyntaxOnly` keep the sources to what Node's type stripping runs in `dev`.
- `.env`: `DATABASE_URL`, which Reylo reads for its analysis (the type check, the editor, the build and the preload) and the app reads through `--env-file-if-exists`.
- `queries/*.sql`: one statement per file, with `:name` parameters, imported from `src/server.ts` with a relative path. `get_user.sql` resolves to a row or `null`, `list_posts.sql` to an array, and `create_post.sql`, an insert, to `void`, since MySQL has no `returning`; `last_insert_id.sql` reads the new id on the same connection.
- `.vscode/settings.json` and `.vscode/extensions.json`: the TypeScript 7 extension, pointed at this project's TypeScript, so the editor shows the queries' types and errors ([Editors](https://reylo.dev/#editors)). `.zed/settings.json` does the same in Zed.

## In your own project

- Install from npm instead of the `link:` dependencies: `npm install reylo mysql2` and `npm install -D @reylosql/mysql typescript@beta`.
- `.npmrc` (`minimumReleaseAge=0`) exists only because this machine holds fresh releases back; drop it.

## Notes

- ts-loader does not work with TypeScript 7: the `typescript` package of the Go port has no JavaScript API, and ts-loader fails with `TypeError: Cannot read properties of undefined (reading 'fileExists')`. swc-loader only strips the types, and `tsc` checks them.
- `last_insert_id()` is a `BIGINT UNSIGNED`, which Reylo gives as a `bigint`. `JSON.stringify` refuses a BigInt, so `send` writes one as its text, and the `POST` answers `{"id":"3"}`.
- A build where the database is out of reach reads `.reylo/`; run `reylo generate` where it can be reached first ([In CI](https://reylo.dev/#in-ci)).
- `typescript` is pinned to `7.1.0-beta`: a range such as `^7.1.0-beta` also takes the nightlies, `7.1.0-dev.*`, which semver orders above the beta.
