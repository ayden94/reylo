import type { applyMdxPreset } from 'fumadocs-mdx/config';

type Environment = Parameters<ReturnType<typeof applyMdxPreset>>[0];

/**
 * docs-kit's MDX options (each section's Markdown on its heading, Twoslash with its cache and the
 * `// @hover:` directive, the hanging indent of wrapped code, each distinct popup built once), with
 * Reylo's own: the modules Reylo's mapper produced for the page's `.sql` blocks, put above each
 * importing block (lib/remark-mapper.ts), and Twoslash's options (lib/twoslash.ts).
 */
export async function mdxOptions(environment: Environment) {
  const { docsMdxOptions } = await import('docs-kit/mdx');
  const { remarkMapper } = await import('./remark-mapper.ts');
  const { reyloTwoslash } = await import('./twoslash.ts');
  return docsMdxOptions(environment, { remarkPlugins: [remarkMapper], twoslash: reyloTwoslash() });
}
