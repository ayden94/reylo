import { readFile } from 'node:fs/promises';
import { join } from 'node:path';
import { siteUrl } from '@/lib/layout.shared';
import { toAgentMarkdown } from '@/lib/remark-sections';

// The whole page as Markdown, for a coding agent or a language model (https://llmstxt.org): the
// page's source, read as each heading's copy button reads its section (lib/remark-sections.ts),
// under the title and the description, with each link to a section of the page made absolute.
// Built once, with the page.
export const dynamic = 'force-static';

export async function GET() {
  const source = await readFile(join(process.cwd(), 'content/docs/index.mdx'), 'utf8');
  const [, frontmatter = '', body = source] = source.match(/^---\n([\s\S]*?)\n---\n([\s\S]*)$/) ?? [];
  const field = (name: string) => frontmatter.match(new RegExp(`^${name}: (.*)$`, 'm'))?.[1] ?? '';
  const markdown = toAgentMarkdown(body).replaceAll('](#', `](${siteUrl}/#`);
  const text = `# ${field('title')}\n\n> ${field('description')}\n\n${markdown}`;
  return new Response(text, { headers: { 'Content-Type': 'text/markdown; charset=utf-8' } });
}
