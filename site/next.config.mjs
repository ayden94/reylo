import { createMDX } from 'fumadocs-mdx/next';

/** @type {import('next').NextConfig} */
const config = {
  reactStrictMode: true,
  // next dev would otherwise write AGENTS.md and CLAUDE.md here.
  agentRules: false,
  devIndicators: false,
  // The repository root has a lockfile of its own; the site is a project by itself.
  turbopack: { root: import.meta.dirname },
  // Twoslash (lib/twoslash.ts) loads TypeScript at build time; it is never bundled.
  serverExternalPackages: ['typescript', 'twoslash'],
};

const withMDX = createMDX();
const mdxConfig = withMDX(config);

// In `next dev`, the compiled page is ignore-listed, so React keeps none of its stack frames, and
// made smaller: without this the dev server ran out of memory on the first render after an edit
// (lib/mdx-dev-loader.mjs). Loaders run last to first, so this one runs after fumadocs-mdx's.
if (process.env.NODE_ENV === 'development') {
  mdxConfig.turbopack.rules['*.{md,mdx}'].loaders.unshift({ loader: './lib/mdx-dev-loader.mjs' });
}

export default mdxConfig;
