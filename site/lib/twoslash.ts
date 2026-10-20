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

/**
 * A block with a `// @hover: <names>` line, written as Twoslash writes its own flags, keeps the
 * popups of the identifiers it names, every occurrence of each, and no other: the rest of its code
 * renders as plain highlighted code, with no popup and no dotted underline. The names are separated
 * by spaces or commas, and several such lines add up. A block without the line keeps every popup.
 * Errors and `^?` queries are kept whatever the line says, since they are not hovers.
 *
 * The line is taken out before Twoslash sees the code, not left to Twoslash as a custom tag:
 * Shiki's transformer passes Twoslash its own default tags on every call, which win over a
 * `customTags` given here, so Twoslash would refuse `@hover` as an unknown compiler flag.
 */
const hoverLine = /^\/\/\s?@hover:\s?(.*)\n?/gm;

function hoverNames(code: string): { code: string; names?: Set<string> } {
  const lines = [...code.matchAll(hoverLine)];
  if (!lines.length) return { code };
  const names = new Set(lines.flatMap((line) => line[1].split(/[\s,]+/).filter(Boolean)));
  return { code: code.replace(hoverLine, ''), names };
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
  const transformer = transformerTwoslash({
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
  // The `@hover` line (`hoverNames` above) comes out before Twoslash runs, and Twoslash's result is
  // narrowed after it is read from the cache or made, so the cache keeps every block's whole result
  // and stays correct whatever the line says. The narrowing comes before Shiki splits the tokens at
  // each node and the renderer wraps them: the transformer keeps the result it renders on
  // `this.meta.twoslash`, the same object its later hooks read.
  const preprocess = transformer.preprocess!;
  transformer.preprocess = function (source, options) {
    const { code, names } = hoverNames(source);
    const result = preprocess.call(this, code, options);
    const twoslash = this.meta.twoslash;
    if (names && twoslash) twoslash.nodes = twoslash.nodes.filter((node) => node.type !== 'hover' || names.has(node.target));
    return result;
  };
  return transformer;
}
