// Runtime half of the experiment: make `import x from "./file.sql"` work in Node
// via the synchronous module hooks API. Loaded with `node --import lando/register`.
import { registerHooks } from "node:module";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

registerHooks({
  load(url, context, nextLoad) {
    if (!url.endsWith(".sql")) return nextLoad(url, context);
    const sql = readFileSync(fileURLToPath(url), "utf8");
    const source = [
      `export const sql = ${JSON.stringify(sql)};`,
      `export const probe = ${JSON.stringify({ runtime: "node-registerHooks", url })};`,
    ].join("\n");
    return { format: "module", source, shortCircuit: true };
  },
});
