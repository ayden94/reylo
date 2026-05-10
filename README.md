# Lando

A TypeScript ORM with first-class polymorphism, single-statement nested
queries, and a schema source you already have. See
[`whitepaper.md`](./whitepaper.md) for the design and
[`TODO.md`](./TODO.md) for the build plan.

## Status

Pre-v0.1. The package layout and tooling are wired up; nothing is shippable
yet. v0.1 targets Postgres only; SQLite arrives in v0.2.

## Layout

A single ESM package with subpath imports. Each subpath corresponds to a
public surface that will eventually be split into its own package if/when
the product calls for it.

| Subpath           | Purpose                                                       |
| ----------------- | ------------------------------------------------------------- |
| `lando`           | ORM root: query client, type inference, SQL compiler.         |
| `lando/schema`    | Schema artifact format, validators, walking helpers.          |
| `lando/schema-pg` | Postgres introspection adapter for the schema kit.            |
| `lando/cli`       | The `lando` CLI: `init`, `generate`, `check`, `diff`, `lock`. |
| `lando/lando-pg`  | Postgres driver adapter for the ORM.                          |

Internal cross-module imports use Node subpath imports (`#schema`,
`#orm`, `#test/*`, etc.) declared in `package.json`.

## Development

Requires Node ≥ 20 and a local Postgres reachable at
`postgres://localhost:5432/postgres` (or set `LANDO_TEST_ADMIN_URL`).

```bash
pnpm install
pnpm typecheck
pnpm test
```

Integration tests create ephemeral databases against the admin URL and
drop them on teardown — no Docker required.
