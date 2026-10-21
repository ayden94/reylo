// Statements reylo does not type. `tsc -p lab/pg-forms/untyped --runExternalCode` must fail with
// exactly one `reylo10` error on each of the first two files, on the statement's own text, and one
// `reylo3` error on repeated.sql, at the line and column of the second :q: each parameter appears
// once in a statement.
import cursor from "./cursor.sql";
import doBlock from "./do_block.sql";
import repeated from "./repeated.sql";
export { cursor, doBlock, repeated };
