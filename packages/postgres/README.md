# @reylosql/postgres

Reylo's analysis engine for Postgres, built to WebAssembly: `engine.wasm`, which [reylo](https://www.npmjs.com/package/reylo) loads to type the `.sql` files of a project whose database is Postgres. It is the engine alone; the type check, the bundler plugins, the run time and the CLI are reylo's.

```sh
npm install reylo
npm install -D @reylosql/postgres
```

Install it at the version of reylo: reylo refuses an engine of another version. A project with more than one database installs the engine of each.
