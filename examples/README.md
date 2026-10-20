# Reylo examples

One complete, working project for each setup the [docs](https://reylo.dev/#setup) describe. Each imports `.sql` files as typed queries, type-checks them against a live database, and runs them in every mode its tool has: a dev server, a production build, a bundle, a test runner. They are the models to copy when you set Reylo up in a project of your own, and each README's "How Reylo is wired in" section lists every line Reylo needs.

| Example | Setup | Database and driver | What runs |
| --- | --- | --- | --- |
| [node](./node) | `node --import reylo/register` ([Node](https://reylo.dev/#node)) | Postgres, postgres.js | a script, and tests with Node's test runner |
| [nub](./nub) | the preload in `nub.jsonc` ([nub](https://reylo.dev/#setup-nub)) | SQLite, `node:sqlite` | a script, and tests with `nub --test` |
| [bun](./bun) | the preload in `bunfig.toml`, and `Bun.build` with `reylo/esbuild` ([Bun](https://reylo.dev/#setup-bun)) | SQLite, `bun:sqlite` | a script, `bun test`, and a bundle |
| [vite](./vite) | `reylo/vite` in a React Router app ([Vite](https://reylo.dev/#setup-vite)) | Postgres, pg | the dev server, the production server, and Vitest |
| [nextjs](./nextjs) | `reylo/webpack-loader` under Turbopack and webpack ([Next.js](https://reylo.dev/#setup-nextjs)) | Postgres, postgres.js | an App Router app: `next dev`, and `next build` and `next start` with each bundler |
| [esbuild](./esbuild) | `reylo/esbuild`: a standalone ES module ([esbuild](https://reylo.dev/#setup-esbuild)) | Postgres, postgres.js | an HTTP server, bundled into one file that runs with plain `node`, and built without a database from `.reylo/` |
| [webpack](./webpack) | `reylo/webpack-loader` ([webpack](https://reylo.dev/#setup-webpack)) | MySQL, mysql2 | an HTTP server, bundled into one ES module |
| [rspack](./rspack) | `reylo/webpack-loader` ([Rspack](https://reylo.dev/#setup-rspack)) | Postgres, pg | an HTTP server, bundled into one ES module |
| [jest](./jest) | `reylo/jest` ([Jest](https://reylo.dev/#setup-jest)) | SQLite, better-sqlite3 | Jest, in an ES module run and a CommonJS run |

Every example has the same two tables, `users` and `posts`, and the same three queries: `get_user.sql` (a row or `null`), `list_posts.sql` (an array) and `create_post.sql` (an insert). Every one also carries the editor settings of [Editors](https://reylo.dev/#editors) for VS Code, Cursor and Zed.

## Running one

Reylo is not on npm yet, so each example links the packages built from this repository, `dist/reylo` and `dist/<database>`, as `link:` dependencies. Build them once at the repository root, and again after a change to `src/`:

```sh
nub run build            # writes dist/
cd examples/<name>
nub install              # or pnpm install; npm does not take link: dependencies
```

Then follow the example's README: it names the database to point `DATABASE_URL` at, the setup that creates the tables, and each command with what it prints. `scripts/gates.sh` runs every example this way, each against a database of its own (`scripts/gates.sh --only example`).

## In your own project

Install from npm instead of linking: `npm install reylo`, and `npm install -D @reylosql/<database> typescript@beta`. Each README lists anything else that exists only because the example lives in this repository: `.npmrc` (`minimumReleaseAge=0`, because this machine holds fresh releases back), and in Next.js the Turbopack root.

The examples pin TypeScript to `7.1.0-beta` exactly: a range such as `^7.1.0-beta`, which `npm install -D typescript@beta` writes, also takes the 7.1 nightlies, which sort above the beta.
