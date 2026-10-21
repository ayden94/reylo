import { defineCollections, defineConfig, defineDocs } from 'fumadocs-mdx/config';

// The pages are content/docs: the main page, index.mdx, and a reference page per dialect. The main
// page goes on in content/sections/, one file per h2 section (lib/page-files.ts), so that an edit
// compiles one section. Both take lib/mdx-options.ts, imported inside the options so that only the
// build loads it (and TypeScript with it).
const mdxOptions = async (environment: Parameters<typeof import('./lib/mdx-options.ts').mdxOptions>[0]) => (await import('./lib/mdx-options.ts')).mdxOptions(environment);

export const docs = defineDocs({ dir: 'content/docs', docs: { mdxOptions } });
export const sections = defineCollections({ type: 'doc', dir: 'content/sections', mdxOptions });

export default defineConfig();
