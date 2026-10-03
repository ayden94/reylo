// The Vite plugin, which is also a Rollup and a Rolldown plugin: one `load` hook that turns a
// `.sql` file into its module (`src/loader.ts`). `enforce: "pre"` runs it before Vite's own
// asset handling; Rollup and Rolldown ignore the field.
import { readFileSync } from "node:fs";
import { releaseConnections } from "./analyze.ts";
import { sqlModule } from "./loader.ts";

export default function motherload() {
  return {
    name: "motherload",
    enforce: "pre" as const,
    async load(this: { addWatchFile(id: string): void }, id: string) {
      if (!id.endsWith(".sql")) return null;
      const { code, entry } = await sqlModule(id, readFileSync(id, "utf8"));
      // A new analysis of the statement (the type check's, after a migration) rebuilds the module.
      this.addWatchFile(entry);
      return { code, map: null };
    },
    async buildEnd() {
      await releaseConnections();
    },
  };
}
