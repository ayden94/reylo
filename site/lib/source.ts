import { defineDocs } from 'fumadocs-mdx/macro';
import { loader } from 'fumadocs-core/source';

// content/docs/index.mdx at / and content/docs/details.mdx at /details.
const docs = defineDocs({
  dir: 'content/docs',
  docs: {
    // Fumadocs' defaults plus Twoslash on `ts twoslash` blocks (lib/twoslash.ts). The macro
    // runs this only at build time, so the app never imports TypeScript.
    mdxOptions: async (environment) => {
      const { applyMdxPreset } = await import('fumadocs-mdx/config');
      const { rehypeCodeDefaultOptions } = await import('fumadocs-core/mdx-plugins');
      const { reyloTwoslash } = await import('./twoslash');
      return applyMdxPreset({
        rehypeCodeOptions: {
          ...rehypeCodeDefaultOptions,
          transformers: [...(rehypeCodeDefaultOptions.transformers ?? []), reyloTwoslash()],
          // Shiki cannot load a language lazily inside a Twoslash popup.
          langs: ['js', 'jsx', 'ts', 'tsx'],
        },
      })(environment);
    },
  },
});

export const source = loader({
  baseUrl: '/',
  source: docs.toFumadocsSource(),
});
