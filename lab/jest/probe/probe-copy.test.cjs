// Which loader handles a .sql file inside a Jest process: Node's own (where porg/register installs
// its hooks) or Jest's sandboxed require. Appends one JSON line per run to ../probe.jsonl. Two
// copies, so that Jest runs them in two workers.
const { appendFileSync } = require("node:fs");
const { join } = require("node:path");
const sql = join(__dirname, "..", "query.sql");

test("where the preload reaches", () => {
  // Jest wraps node:module and process.getBuiltinModule; the static Module._load its wrapper copies
  // is still Node's own loader.
  const viaNode = attempt(() => require("node:module")._load(sql, null, false));
  const viaJest = attempt(() => require(sql));
  appendFileSync(join(__dirname, "..", "probe.jsonl"), JSON.stringify({ pid: process.pid, worker: process.env.JEST_WORKER_ID, script: process.argv[1], execArgv: process.execArgv, NODE_OPTIONS: process.env.NODE_OPTIONS, viaNode, viaJest }) + "\n");
});

function attempt(load) {
  try {
    return `loaded, run is a ${typeof load().default.run}`;
  } catch (e) {
    return `${e.name}: ${e.message.split("\n")[0]}`;
  }
}
