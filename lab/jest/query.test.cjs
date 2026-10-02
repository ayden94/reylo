// CommonJS: Jest's own require() loads the .sql file.
const { DatabaseSync } = require("node:sqlite");
const { join } = require("node:path");
const query = require("./query.sql").default;

test("a .sql file required by a CommonJS test runs", async () => {
  const db = new DatabaseSync(join(__dirname, "lab.db"));
  expect(query.cardinality).toBe("maybeOne");
  expect(await query.run(db, { id: 1 })).toEqual({ id: 1, name: "ada", email: "ada@example.com" });
  expect(await query.run(db, { id: 99 })).toBe(null);
});
