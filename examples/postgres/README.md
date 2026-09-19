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

## See it in VS Code or Cursor

1. Install the **TypeScript 7** extension (`TypeScriptTeam.native-preview`); `.vscode/extensions.json` recommends it. The build must be 0.20260918.1 or later, because content mappers arrived in the extension's bundled tsgo after the July build. On 2026-09-18 Cursor's extension gallery served 0.20260918.1 and the Visual Studio Marketplace still served 0.20260708.2, so in VS Code check the version and, if it is older, install the current build from a VSIX (below).
2. Open this folder as the workspace root and trust it. Content mappers only run in trusted workspaces.
3. Open `src/main.ts` and hover `user` or `posts`. Go to `queries/get_user.sql`, change `name` to `nam`, and watch the error appear in the `.sql` file at the column position.

`.vscode/settings.json` sets `typescript.experimental.useTsgo` (the extension's server replaces the built-in TypeScript) and `js/ts.contentMappers.enabled` (on by default, kept explicit). The extension brings its own tsgo, so no "Select TypeScript Version" step is needed. The mapper reads `DATABASE_URL` from `.env` in this folder. After a change to the mapper's code, run "TypeScript Native Preview: Restart".

**A VSIX of the current build.** Package it from a copy that Cursor (or any editor with the current build) installed, then use "Extensions: Install from VSIX":

```sh
cp -R ~/.cursor/extensions/typescriptteam.native-preview-0.20260918.1-darwin-arm64 /tmp/native-preview
cd /tmp/native-preview && npx @vscode/vsce package --target darwin-arm64 --no-dependencies --allow-missing-repository --allow-unused-files-pattern --skip-license -o native-preview.vsix
```
