# Lando example: Postgres

A sample project that imports `.sql` files as typed queries, checked by TypeScript 7.1 through the `lando` content mapper and run through the `postgres` driver.

## Run it

```sh
psql postgres://localhost:5432/postgres -f setup.sql   # a local Postgres, or point .env at yours
nub install                                             # or npm install
nub run typecheck                                       # tsc --noEmit --runExternalCode
nub run start                                           # node --import lando/register src/main.ts
```

Edit `queries/get_user.sql` (rename a column, add a parameter) and run `nub run typecheck` again: the types in `src/main.ts` follow the database.

## See it in VS Code

1. Install the **TypeScript 7** extension (`TypeScriptTeam.native-preview`); `.vscode/extensions.json` recommends it.
2. Open this folder as the workspace root and trust it. Content mappers only run in trusted workspaces.
3. Accept "Use Workspace Version" when prompted (or run "TypeScript: Select TypeScript Version"), so the editor uses the 7.1 nightly in `node_modules/typescript`.
4. Open `src/main.ts` and hover `user` or `posts`. Go to `queries/get_user.sql`, change `name` to `nam`, and watch the error appear in the `.sql` file at the column position.

`.vscode/settings.json` turns on `js/ts.contentMappers.enabled` (the default) and points `typescript.native-preview.tsdk` at the workspace TypeScript. The mapper reads `DATABASE_URL` from `.env` in this folder.
