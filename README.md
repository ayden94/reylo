# Lando

Import a `.sql` file and get a fully typed query.

```ts
import * as getUser from "./get_user.sql";
```

Lando is a TypeScript 7.1 content mapper that parses the SQL, reconciles it against the real database catalog, and hands the type checker a typed module. A companion runtime loader makes the same import work in Node, nub, and Bun.

Status: pre-alpha. Design notes and experiments live in this repository while the shape settles.
