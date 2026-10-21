import { readFile } from 'node:fs/promises';
import { join } from 'node:path';
import { siteUrl } from '@/lib/layout.shared';
import { toAgentMarkdown } from 'docs-kit/remark-sections';

// Every page as Markdown, for a coding agent or a language model (https://llmstxt.org): the main
// page, then each dialect's reference, in the order a reader takes them. Each is its page's source,
// read as each heading's copy button reads its section (docs-kit's remark-sections), under its
// title and description, with each link made absolute: a link to a section of the same page
// (`#…`) to that page's address, and a link to another page (`/…`) to the site's. Built once,
// with the pages.
export const dynamic = 'force-static';

const PAGES = [
  { file: 'index.mdx', url: '/' },
  { file: 'postgres.mdx', url: '/postgres' },
  { file: 'mysql.mdx', url: '/mysql' },
  { file: 'sqlite.mdx', url: '/sqlite' },
];

async function markdown({ file, url }: (typeof PAGES)[number]): Promise<string> {
  const source = await readFile(join(process.cwd(), 'content/docs', file), 'utf8');
  const [, frontmatter = '', body = source] = source.match(/^---\n([\s\S]*?)\n---\n([\s\S]*)$/) ?? [];
  const field = (name: string) => frontmatter.match(new RegExp(`^${name}: (.*)$`, 'm'))?.[1] ?? '';
  const page = url === '/' ? `${siteUrl}/` : `${siteUrl}${url}`;
  const text = toAgentMarkdown(body).replaceAll('](#', `](${page}#`).replaceAll('](/', `](${siteUrl}/`);
  return `# ${field('title')}\n\n> ${field('description')}\n\n${text.trim()}\n`;
}

export async function GET() {
  const text = (await Promise.all(PAGES.map(markdown))).join('\n');
  return new Response(text, { headers: { 'Content-Type': 'text/markdown; charset=utf-8' } });
}
