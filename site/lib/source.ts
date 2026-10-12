import { defineDocs } from 'fumadocs-mdx/macro';
import { loader } from 'fumadocs-core/source';

/** A hast node, as much of one as the hanging indent reads. */
type HastNode = { type: string; value?: string; children?: HastNode[]; properties?: Record<string, unknown> };

const textOf = (node: HastNode): string => (node.type === 'text' ? (node.value ?? '') : (node.children ?? []).map(textOf).join(''));

// Each code line carries its own indent as `--indent`, so a line that wraps on a narrow screen
// hangs two characters in from where the line starts, not from the block's edge (app/global.css,
// "Code never scrolls sideways").
const hangingIndent = {
  name: 'reylo:hanging-indent',
  line(node: HastNode) {
    const indent = /^ */.exec(textOf(node))![0].length;
    if (!indent) return;
    const style = [node.properties?.style, `--indent: ${indent}ch`].filter(Boolean).join('; ');
    node.properties = { ...node.properties, style };
  },
};

// The site is one page, content/docs/index.mdx, at /.
const docs = defineDocs({
  dir: 'content/docs',
  docs: {
    // Fumadocs' defaults plus Twoslash on `ts twoslash` blocks (lib/twoslash.ts), whose imports of
    // the page's `.sql` blocks are typed by the modules Reylo's mapper produced for them
    // (lib/remark-mapper.ts), each section's Markdown on its heading (lib/remark-sections.ts), and
    // each distinct Twoslash popup built once (lib/recma-shared-popups.ts).
    // The macro runs this only at build time, so the app never imports TypeScript.
    mdxOptions: async (environment) => {
      const { applyMdxPreset } = await import('fumadocs-mdx/config');
      const { rehypeCodeDefaultOptions } = await import('fumadocs-core/mdx-plugins');
      const { reyloTwoslash } = await import('./twoslash');
      const { remarkMapper } = await import('./remark-mapper');
      const { remarkSections } = await import('./remark-sections');
      const { recmaSharedPopups } = await import('./recma-shared-popups');
      return applyMdxPreset({
        remarkPlugins: [remarkSections, remarkMapper],
        recmaPlugins: [recmaSharedPopups],
        rehypeCodeOptions: {
          ...rehypeCodeDefaultOptions,
          transformers: [...(rehypeCodeDefaultOptions.transformers ?? []), reyloTwoslash(), hangingIndent],
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
