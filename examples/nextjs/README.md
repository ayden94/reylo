# Reylo example: Next.js

A Next.js 16.4 App Router app whose page, server action and route handler run `.sql` files as typed queries against Postgres through postgres.js, built with Turbopack or webpack. It is the setup of [Next.js](https://reylo.dev/#setup-nextjs) in the docs, on the app `create-next-app` generates (TypeScript, App Router, no Tailwind, no ESLint).

## Run it

```sh
nub run build                 # at the repository root, once: writes dist/, which this example links
cd examples/nextjs
nub install                   # or pnpm install; npm does not take the link: dependencies
createdb reylo_nextjs         # any Postgres database you can drop tables in
cp .env.example .env          # DATABASE_URL=postgres://localhost:5432/reylo_nextjs
nub run setup                 # creates users and posts: "created users and posts"
nub run typecheck             # next typegen, then tsc --noEmit --runExternalCode: "✓ Types generated successfully", exit 0
nub run dev                   # next dev (Turbopack) on http://localhost:3000; nub run dev:webpack for webpack
nub run build                 # the type check, then next build (Turbopack): "◐ /" and "ƒ /api/users/[id]"
nub run start                 # next start on http://localhost:3000
nub run build:webpack         # the same build with webpack; then nub run start
```

With `dev` or `start` running, `/` lists the posts with a form that adds one, and:

```text
$ curl localhost:3000/api/users/1
{"id":1,"name":"ada","email":"ada@example.com","role":"admin","created_at":"2026-10-20T23:17:58.136786Z"}
$ curl -w ' %{http_code}\n' localhost:3000/api/users/99
{"error":"not found"} 404
```

While `nub run dev` or `nub run dev:webpack` runs, an edit to a `.sql` file is picked up on the next request, with no restart: change `order by p.id` to `order by p.id desc` in `queries/list_posts.sql` and reload `/`.

Misspell a column in `queries/list_posts.sql` and `nub run typecheck` reports it at the file's line and column, and `nub run build` stops there:

```text
queries/list_posts.sql(1,14): error reylo6: column p.titel does not exist (42703)
```

`next dev` and `next build` alone report the same error from the loader, as `queries/list_posts.sql:1:14: column p.titel does not exist (42703)`.

## How Reylo is wired in

- `package.json`: `reylo` is a dependency (the loader, and the run time each query module imports); `@reylosql/postgres` (the engine that analyzes each statement) and `typescript` 7.1 beta are dev dependencies. `postgres` is the driver the app runs the queries with, and the type check reads the schema through it too.
- `package.json` scripts: `typecheck` is `next typegen && tsc --noEmit --runExternalCode`: `next typegen` writes the route types (`LayoutProps`, `RouteContext`) the app uses, and `--runExternalCode` lets `tsc` run the content mapper. `build` runs the same check before `next build`, whose own type check cannot run content mappers (below). `dev`, `start` and the webpack variants `dev:webpack` and `build:webpack` are Next's own commands.
- `next.config.ts`: `turbopack.rules` sends `*.sql` through `reylo/webpack-loader`, with `condition: { not: "browser" }` so only server code may import a query, and `as: "*.js"` so Turbopack treats the loader's output as JavaScript. The `webpack` hook adds the same loader on the server only, for `--webpack`. `typescript.ignoreBuildErrors` turns off `next build`'s type check, which runs the project's `tsc` without `--runExternalCode` and so fails on every `.sql` import (TS2307, and TS18068 on `tsconfig.json`); the `build` script has run the real check by then. `turbopack.root` is there only because Reylo is linked from this repository (below).
- `tsconfig.json`: `contentMappers` names `reylo` for `.sql`; the rest is what `create-next-app` writes. Its `include` takes `.ts` and `.tsx` files only, so `setup.sql`, a script rather than one query, stays out of the type check, and the queries come in through their imports.
- `queries/*.sql`: one statement per file, with `:name` parameters, imported as `@/queries/<name>.sql` through the template's `@/*` path. They sit in a top-level `queries/` beside `lib/` rather than in `app/`, because they are the app's data layer, shared by routes, and no route segment owns them. `get_user.sql` resolves to a row or `null`, `list_posts.sql` to an array, and `create_post.sql`, an insert with `returning`, to the row it wrote.
- `lib/db.ts`: the one postgres.js client every query runs on, marked `server-only`, and kept on `globalThis` so that `next dev` does not open another pool each time it evaluates the module again.
- `app/page.tsx`: a server component that runs `list_posts.sql`, and a server action, `addPost`, that runs `create_post.sql` and calls `revalidatePath("/")`. `app/api/users/[id]/route.ts`: a route handler that runs `get_user.sql` and answers 404 for `null`. Only server code imports a query: a client component (`"use client"`) that imports one fails the build (below).
- `.env`: `DATABASE_URL`, which Reylo reads for its analysis (the type check, the editor and the loader) and Next.js loads for the app. `setup` loads it with `node --env-file-if-exists=.env`. A `DATABASE_URL` in the environment wins over `.env` in each.
- `.vscode/settings.json` and `.vscode/extensions.json`: the TypeScript 7 extension, pointed at this project's TypeScript, so the editor shows the queries' types and errors ([Editors](https://reylo.dev/#editors)). `.zed/settings.json` does the same in Zed.

## In your own project

- Install from npm instead of the `link:` dependencies: `npm install reylo postgres` and `npm install -D @reylosql/postgres typescript@beta`.
- Drop `turbopack.root` from `next.config.ts`. Turbopack resolves nothing outside its root, and the `link:` dependency's real path is this repository's `dist/`, outside this directory; an app that installs `reylo` keeps the default root.
- `.npmrc` (`minimumReleaseAge=0`) exists only because this machine holds fresh releases back; drop it.

## Notes

- **Cache Components.** `create-next-app` 16.4 turns on `cacheComponents` (and `partialPrefetching`), and this example keeps them. Under Cache Components, a component that queries the database at request time must sit inside `<Suspense>`, as `Posts` does in `app/page.tsx`: without it, `next build` stops with "Next.js encountered uncached or runtime data during prerendering". The page then prerenders its shell and streams the list (`◐ /` in the build output). To prerender the list instead, mark the component `"use cache"`; the build then queries the database.
- **A query in client code.** A `"use client"` component that imports a `.sql` file fails `next build` and the page in `next dev`: Turbopack says `queries/get_user.sql` is an "Unknown module type", webpack that it needs "an appropriate loader". The type check does not catch it, since types know nothing of the client boundary; keep queries behind server code, as `lib/db.ts`'s `server-only` keeps the client.
- **The server action from curl.** A form whose `action` is a server action works without JavaScript: the page carries a hidden `$ACTION_ID_<id>` field, and a multipart POST of it with the form's fields runs the action. `curl -H 'Origin: http://localhost:3000' -F '$ACTION_ID_<id>=' -F user_id=2 -F title=Hi -F body= localhost:3000/`, with the id read from the page, adds a post.
- **`server-only`.** Next.js handles `import "server-only"` itself and declares its types, so the package need not be installed.
- **`AGENTS.md`** is the file `next dev` writes for coding agents (turn it off with `agentRules: false`); it is kept so that running the app leaves no new file.
