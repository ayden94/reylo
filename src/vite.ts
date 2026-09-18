// The Vite plugin, which is also a Rollup and a Rolldown plugin: one `load` hook that turns a
// `.sql` file into the module `lando/register` would have made at runtime. `enforce: "pre"`
// runs it before Vite's own asset handling; Rollup and Rolldown ignore the field.
import { readFileSync } from "node:fs";
import { sqlModuleSource } from "./loader.ts";

export default function lando() {
  return {
    name: "lando",
    enforce: "pre" as const,
    load(id: string) {
      if (!id.endsWith(".sql")) return null;
      return { code: sqlModuleSource(readFileSync(id, "utf8"), id), map: null };
    },
  };
}
