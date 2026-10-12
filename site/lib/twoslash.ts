import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { defaultHoverInfoProcessor } from '@shikijs/twoslash';
import { transformerTwoslash } from 'fumadocs-twoslash';
import { createFileSystemTypesCache } from 'fumadocs-twoslash/cache-fs';

// Twoslash runs TypeScript's JavaScript API (6.0, from fumadocs-twoslash), which cannot run a
// content mapper. So a `ts twoslash` block carries, as a hidden file, the module reylo's mapper
// returns for the `.sql` block it imports (`get_user.d.sql.ts`, which `./get_user.sql` resolves to
// under allowArbitraryExtensions; lib/remark-mapper.ts puts it there from lib/sql-modules.json),
// and the `reylo` package that module imports is mounted here from the repository's own source:
// the hovers show the real `Query` type and its docs.

/** reylo's root export and every module it reaches through relative imports. */
function reyloFiles(repo: string): Record<string, string> {
  const files: Record<string, string> = {
    'node_modules/reylo/package.json': readFileSync(path.join(repo, 'package.json'), 'utf8'),
  };
  const pending = ['src/index.ts'];
  for (let file = pending.pop(); file; file = pending.pop()) {
    const key = `node_modules/reylo/${file}`;
    if (key in files) continue;
    const text = readFileSync(path.join(repo, file), 'utf8');
    files[key] = text;
    // Only the import declarations: reylo names its own modules with their .ts extension.
    for (const [, spec] of text.matchAll(/^(?:import|export)\b[^;]*?\sfrom\s+["'](\.{1,2}\/[^"']+\.ts)["']/gm)) {
      pending.push(path.posix.join(path.posix.dirname(file), spec));
    }
  }
  return files;
}

/** The options every block is checked with; scripts and tests reach Twoslash through these too. */
export function twoslashOptions() {
  // The build runs in site/; the reylo package is the repository root above it.
  const repo = path.resolve(process.cwd(), '..');
  return {
    extraFiles: reyloFiles(repo),
    compilerOptions: {
      // `process.env` and node:sqlite in the examples; @types/node is a dev dependency of the
      // site. A block that uses Bun or Cloudflare D1 references their types itself
      // (lib/remark-mapper.ts adds the reference), so no other block sees their globals.
      types: ['node'],
      // `import getUser from "./get_user.sql"` resolves to the hidden `get_user.d.sql.ts`.
      allowArbitraryExtensions: true,
      // reylo's source imports its own modules with their .ts extension, as the repository's
      // tsconfig allows; nothing on the site uses twoslash's emit.
      allowImportingTsExtensions: true,
      noEmit: true,
      // The drivers' declarations are not the page's to check: Bun's and Cloudflare's globals
      // overlap Node's and the DOM's.
      skipLibCheck: true,
    },
  };
}

export function reyloTwoslash() {
  const options = twoslashOptions();
  // Each block's result is kept on disk under a hash of its code, which holds its hidden setup (the
  // captured module), so `next dev` checks again only the blocks an edit changed: without it every
  // compile checked every block, and the dev server's memory grew by gigabytes per edit until it
  // stopped. The directory is named for everything else a result depends on (the reylo source the
  // blocks import, the compiler options, the site's dependencies), so none of it can go stale.
  const site = process.cwd();
  const key = createHash('sha256')
    .update(JSON.stringify(options))
    .update(readFileSync(path.join(site, 'pnpm-lock.yaml'), 'utf8'))
    .digest('hex')
    .slice(0, 12);
  return transformerTwoslash({
    twoslashOptions: options,
    typesCache: createFileSystemTypesCache({ cwd: site, dir: `.next/cache/twoslash/${key}` }),
    rendererRich: {
      // `Query<…the whole query type…>.run(…)` reads as `run(…)`: the query's own hover shows the
      // type, and repeating it made the popup of every call scroll.
      processHoverInfo: (info) =>
        defaultHoverInfoProcessor(info).replace(/^Query<[\s\S]*?>\.(?=(?:run|runSync|with)\b)/, ''),
      // Node's documentation of process.env runs to a screen; the page's hovers are about queries.
      processHoverDocs: (docs) => (docs.length > 400 ? '' : docs),
    },
  });
}
