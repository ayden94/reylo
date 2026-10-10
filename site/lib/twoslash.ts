import { readFileSync } from 'node:fs';
import path from 'node:path';
import { transformerTwoslash } from 'fumadocs-twoslash';

// Twoslash runs TypeScript's JavaScript API (6.0, from fumadocs-twoslash), which cannot run a
// content mapper. So a `ts twoslash` block carries, as a hidden file, the declaration reylo's
// mapper produced for its statement (`get_user.d.sql.ts`, which `./get_user.sql` resolves to
// under allowArbitraryExtensions), and the `reylo` package that declaration imports is mounted
// here from the repository's own source: the hovers show the real `Query` type and its docs.

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

export function reyloTwoslash() {
  // The build runs in site/; the reylo package is the repository root above it.
  const repo = path.resolve(process.cwd(), '..');
  return transformerTwoslash({
    twoslashOptions: {
      extraFiles: reyloFiles(repo),
      compilerOptions: {
        // `process.env` in the examples; @types/node is a dev dependency of the site.
        types: ['node'],
        // reylo's source imports its own modules with their .ts extension, as the repository's
        // tsconfig allows; nothing on the site uses twoslash's emit.
        allowImportingTsExtensions: true,
        noEmit: true,
      },
    },
  });
}
