import path from 'node:path';
import { docsKit } from 'docs-kit/vite';
import { defineConfig } from 'vite';
import { playground } from './lib/playground-plugin.ts';
import { mapperDependents } from './lib/remark-mapper.ts';
import * as mdxConfig from './source.config.ts';

// docs-kit's plugins (docs-kit/src/vite.ts): fumadocs-mdx over source.config.ts, Tailwind 4, React
// Router, and what keeps `vite dev` cheap on a large page (one compile per edit, a small module, a
// section re-rendered alone), and the mapper's own (a section compiled again when a statement it
// imports from another section changes, and only then), and the local playground's endpoint on the
// dev server (lib/playground-plugin.ts). `@/` is the site's root; `collections` is fumadocs-mdx's
// generated .source/ (lib/source.ts).
export default defineConfig({
  plugins: [...docsKit(mdxConfig), mapperDependents(), playground()],
  resolve: {
    alias: {
      '@': import.meta.dirname,
      collections: path.join(import.meta.dirname, '.source'),
    },
  },
  server: { port: 3311 },
  // @vercel/og (the link preview, app/routes/opengraph-image.tsx) loads its WebAssembly with a
  // `require` Vite's module runner cannot run, so Node loads it.
  ssr: { external: ['@vercel/og'] },
});
