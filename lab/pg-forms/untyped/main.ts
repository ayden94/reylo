// Statements dotsql does not type. `tsc -p lab/pg-forms/untyped --runExternalCode` must fail with
// exactly one `porg10` error on each file, on the statement's own text.
import cursor from "./cursor.sql";
import doBlock from "./do_block.sql";
export { cursor, doBlock };
