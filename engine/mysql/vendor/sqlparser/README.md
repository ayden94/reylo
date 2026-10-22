# sqlparser, the MySQL engine's copy

This is [`sqlparser`](https://github.com/apache/datafusion-sqlparser-rs) 0.63.0, the Apache DataFusion SQL parser, as published on crates.io, under the Apache License 2.0 (`LICENSE.TXT`). The MySQL engine reads a statement's tree with it for what the server's prepare metadata does not say: the row-count proof, the JSON constructors' shapes and the nullability probes.

The manifest leaves out the crate's development dependencies, its optional dependencies (the features stay, as names, so the sources' `cfg`s are known) and its docs.rs metadata, and its default features are `std` alone. Its sources are upstream's without the `#[cfg(test)]` items, which need those development dependencies.

## What this copy changes

The engine reads a query, INSERT (or REPLACE), UPDATE or DELETE, and parses nothing else, and its only dialect is MySQL's. The copy is cut to that, which takes the MySQL engine's artifact from 2.67 to 1.69 MB with every analysis unchanged. Each file it changes says so under its license header.

- `Statement` keeps the variants `Query`, `Insert`, `Update` and `Delete`, and is `#[non_exhaustive]`. Every match arm, parse function, conversion and dialect hook that named another variant is gone, and so is what only they used: through `SetExpr`, which can hold a `Statement`, every DDL type was reachable from a query, with its printing, cloning, comparing and dropping.
- `Parser::parse_statement` reads those statements alone, and a query can no longer be a MERGE.
- `dialect_of!` and `dialect_is!` are decided for `MySqlDialect` at compile time, so the parser's branches for other dialects are compiled out. The engine's dialect reports MySqlDialect's `TypeId`, as every dialect check saw before.
- A `Keyword` prints from `ALL_KEYWORDS` instead of a 65 KB match with an arm per keyword.
- `INSERT` no longer reads Hive's `OVERWRITE [LOCAL] DIRECTORY`, so `INSERT INTO directory …` and `INSERT INTO local …`, which upstream refuses, name a table as they do in MySQL.
