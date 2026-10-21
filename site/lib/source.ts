import { defineDocs } from 'fumadocs-mdx/macro';
import { loader } from 'fumadocs-core/source';

// The site's pages are content/docs: the main page, index.mdx, at /, and a reference page per
// dialect, postgres.mdx, mysql.mdx and sqlite.mdx, at /postgres, /mysql and /sqlite. Their MDX
// options are lib/mdx-options.ts, imported inside the options, which the macro runs only at build
// time, so the app never bundles them or TypeScript.
const docs = defineDocs({
  dir: 'content/docs',
  docs: {
    mdxOptions: async (environment) => (await import('./mdx-options')).mdxOptions(environment),
  },
});

export const source = loader({
  baseUrl: '/',
  source: docs.toFumadocsSource(),
});
