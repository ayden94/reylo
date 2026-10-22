# sqlparser, the MySQL engine's copy

This is [`sqlparser`](https://github.com/apache/datafusion-sqlparser-rs) 0.63.0, the Apache DataFusion SQL parser, as published on crates.io, under the Apache License 2.0 (`LICENSE.TXT`). The MySQL engine reads a statement's tree with it for what the server's prepare metadata does not say: the row-count proof, the JSON constructors' shapes and the nullability probes.

The manifest leaves out the crate's development dependencies, its optional dependencies (the features stay, as names, so the sources' `cfg`s are known) and its docs.rs metadata, and its default features are `std` alone. Its sources are upstream's without the `#[cfg(test)]` items, which need those development dependencies.
