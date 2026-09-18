# Bundlers lab

One `main.ts` (and its `main.mjs` twin for the bundlers that take no TypeScript entry) importing `query.sql`, bundled through every adapter Lando ships, with `lando` and `postgres` external so each bundle runs against the local Postgres afterwards:

| Adapter | Bundlers proven | Command |
| --- | --- | --- |
| `lando/vite` | Vite 7, Rollup 4, Rolldown 1 | `vite build`, `rollup -c rollup.config.mjs`, `rolldown -c rolldown.config.mjs` |
| `lando/esbuild` | esbuild 0.28, `Bun.build`, `Bun.plugin` (preload) | `node esbuild.mjs`, `bun bun-build.ts`, `bun --preload ./bun-preload.ts main.ts` |
| `lando/webpack-loader` | webpack 5, Rspack 1 (Turbopack takes the same loader file in `turbopack.rules`, not run here) | `webpack --config webpack.config.mjs`, `rspack build --config rspack.config.mjs` |

Then `node dist/<bundler>/main.js` prints the row. `nub install` first (a `node_modules/lando` symlink to the repo root is created by hand: `ln -s ../../.. node_modules/lando`). The `.lando/queries.json` here is a copy of `lab/basic`'s, so the bundles carry the engine's rewritten statement; without it the loader falls back to lexing the placeholders itself.
