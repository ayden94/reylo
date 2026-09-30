// The Vite plugin, which is also a Rollup and a Rolldown plugin: one `load` hook that turns a
// `.sql` file into the module `porg/register` would have made at runtime. `enforce: "pre"`
// runs it before Vite's own asset handling; Rollup and Rolldown ignore the field.
import { readFileSync } from "node:fs";
import { cacheFile, sqlModuleSource } from "./loader.ts";

export default function porg() {
  return {
    name: "porg",
    enforce: "pre" as const,
    load(this: { addWatchFile(id: string): void }, id: string) {
      if (!id.endsWith(".sql")) return null;
      // The module carries the mapper's rewritten statement, so a new cache must reload it.
      const cache = cacheFile(id);
      if (cache) this.addWatchFile(cache);
      return { code: sqlModuleSource(readFileSync(id, "utf8"), id), map: null };
    },
  };
}
