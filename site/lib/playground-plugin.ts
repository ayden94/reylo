import { statSync } from 'node:fs';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import type { Plugin } from 'vite';

/**
 * The local playground's endpoint, on the dev server only: `GET /__playground?q=<statement>`, with
 * `dialects=postgres,mysql,sqlite` to try it on fewer and `schema=<sql>` for a schema of the card's
 * own, answers the card's compiled MDX (lib/playground.ts), which app/routes/playground.tsx draws.
 * Node loads lib/playground.ts itself, as `nub run capture` loads the same modules, so its scratch
 * databases and sessions last as long as the server, or until the file changes.
 */
export function playground(): Plugin {
  return {
    name: 'reylo-playground',
    apply: 'serve',
    configureServer(server) {
      server.middlewares.use('/__playground', async (req, res) => {
        const query = new URL(req.url ?? '/', 'http://localhost').searchParams;
        const dialects = (query.get('dialects') ?? '').split(',').filter((d): d is 'postgres' | 'mysql' | 'sqlite' => d === 'postgres' || d === 'mysql' || d === 'sqlite');
        res.setHeader('content-type', 'application/json');
        try {
          // Loaded again after an edit to it, which starts its sessions again.
          const file = path.join(server.config.root, 'lib', 'playground.ts');
          const { card } = (await import(`${pathToFileURL(file).href}?${statSync(file).mtimeMs}`)) as typeof import('./playground.ts');
          res.end(JSON.stringify(await card({ sql: query.get('q') ?? '', dialects, schema: query.get('schema') ?? undefined })));
        } catch (e) {
          res.statusCode = 500;
          res.end(JSON.stringify({ errors: {}, failure: (e as Error).stack ?? String(e) }));
        }
      });
    },
  };
}
