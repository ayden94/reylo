// Gives each h2 and h3 its section as Markdown, for the heading's copy button
// (components/heading.tsx): the text from the heading to the next heading of the same or a
// higher level, taken from the page's source, with what only the page needs removed, so the
// section pastes cleanly into an agent. Ported from Motherload's lib/remark-sections.ts, with
// Reylo's components: code-tab groups, <DialectTabs>/<Dialect>, <Accordions>/<Accordion>,
// <Details>, and the module a Twoslash block hides above its `// ---cut---` line. The source is read before
// lib/remark-mapper.ts expands an example's TypeScript into a panel per dialect, so a copied
// section shows that TypeScript once, as it is written.

type Node = {
  type: string;
  depth?: number;
  position?: { start: { offset?: number } };
  data?: { hProperties?: Record<string, unknown> };
  children?: Node[];
};

const FENCE = /^```[^\n]*\n[\s\S]*?^```$/gm;

/** A fence's meta attribute, such as `tab="Postgres"`. */
const attribute = (meta: string, name: string) => meta.match(new RegExp(`(?:^|\\s)${name}="([^"]*)"`))?.[1];

/**
 * One fenced block as an agent should read it: the language alone on the fence, a label above it
 * for its tab and file name, and none of Twoslash's setup, directives and queries or the diff
 * markers.
 */
function cleanBlock(block: string): string {
  const newline = block.indexOf('\n');
  const [, lang = '', meta = ''] = block.slice(0, newline).match(/^```(\S*)\s*(.*)$/) ?? [];
  let body = block.slice(newline + 1, block.lastIndexOf('```'));
  // The setup a Twoslash block hides: everything above `---cut---` (or `---cut-before---`) and
  // everything below `---cut-after---`.
  body = body.replace(/^[\s\S]*?^\/\/ ---cut(?:-before)?---\n/m, '').replace(/^\/\/ ---cut-after---\n[\s\S]*$/m, '');
  body = body
    // Twoslash's directives (`// @noErrors`), queries (`//    ^?`) and completions (`^|`).
    .replace(/^[ \t]*\/\/ @\w+.*\n/gm, '')
    .replace(/^[ \t]*\/\/\s*\^[?|][ \t]*\n/gm, '')
    // Shiki's diff and highlight markers.
    .replace(/[ \t]*\/\/ \[!code [^\]]*\]/g, '');
  // A tab named for a dialect is bold; a file name, as a tab or a title, is code.
  const tab = attribute(meta, 'tab');
  const title = attribute(meta, 'title');
  const name = (text: string) => (/\.\w+$/.test(text) ? `\`${text}\`` : `**${text}**`);
  const parts = [tab, title].filter((part, i, all): part is string => !!part && all.indexOf(part) === i);
  const label = parts.length ? `${parts.map(name).join(', ')}:\n\n` : '';
  return `${label}\`\`\`${lang}\n${body}\`\`\``;
}

export function toAgentMarkdown(source: string): string {
  return (
    source
      .replace(FENCE, cleanBlock)
      // A terminal block's colours are escape codes, which an agent reads as noise.
      .replace(/\x1b\[[0-9;]*m/g, '')
      // MDX comments.
      .replace(/\{\/\*[\s\S]*?\*\/\}\n?/g, '')
      // The page's components as plain Markdown.
      .replace(/<Callout[^>]*?title="([^"]*)"[^>]*>\s*([\s\S]*?)\s*<\/Callout>/g, (_, title: string, body: string) => `> **${title}.** ${body.replace(/\n/g, '\n> ')}`)
      .replace(/<Callout[^>]*>\s*([\s\S]*?)\s*<\/Callout>/g, (_, body: string) => `> ${body.replace(/\n/g, '\n> ')}`)
      .replace(/^<(?:Tab|Dialect) value="([^"]*)">$/gm, '**$1**')
      .replace(/^<Accordion title="([^"]*)"[^>]*>$/gm, '**$1**')
      .replace(/^<Details summary="([^"]*)"[^>]*>$/gm, '**$1**')
      .replace(/^<\/?(?:Tabs|Tab|DialectTabs|Dialect|Accordions|Accordion|Details)\b[^>]*>\n?/gm, '')
      // A paragraph written as HTML for its class, such as the lede, as a paragraph.
      .replace(/<p\b[^>]*>([\s\S]*?)<\/p>/g, '$1')
      // Any other line that is only a tag, outside the code blocks: a wrapper such as the feature
      // list's `<div>` or `<AgentPrompt>`, whose contents are Markdown already, or an anchor.
      .split(/(^```[^\n]*\n[\s\S]*?^```$)/m)
      .map((part, i) => (i % 2 ? part : part.replace(/^[ \t]*<\/?[A-Za-z][\w.-]*(?:\s[^<>]*)?\/?>[ \t]*\n?/gm, '')))
      .join('')
      .replace(/\n{3,}/g, '\n\n')
      .trim() + '\n'
  );
}

export function remarkSections() {
  return (tree: Node, file: { value: unknown }) => {
    const source = String(file.value);
    const headings = (tree.children ?? []).filter((node) => node.type === 'heading' && node.position?.start.offset !== undefined);
    headings.forEach((heading, i) => {
      if (heading.depth !== 2 && heading.depth !== 3) return;
      const next = headings.slice(i + 1).find((other) => other.depth! <= heading.depth!);
      const section = source.slice(heading.position!.start.offset, next?.position!.start.offset ?? source.length);
      heading.data ??= {};
      heading.data.hProperties = { ...heading.data.hProperties, 'data-section': toAgentMarkdown(section) };
    });
  };
}
