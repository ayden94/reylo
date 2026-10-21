import { readFile } from 'node:fs/promises';
import path from 'node:path';
import { toAgentMarkdown } from 'docs-kit/remark-sections';
import { siteUrl } from '@/lib/layout.shared';
import { pageFiles } from '@/lib/page-files';

// Every page as Markdown, for a coding agent or a language model (https://llmstxt.org): the main
// page, its files in order (lib/page-files.ts), then each dialect's reference, in the order a
// reader takes them. Each is its page's source, read as each heading's copy button reads its
// section (docs-kit's remark-sections), under its title and description, with each link made
// absolute: a link to a section of the same page (`#…`) to that page's address, and a link to
// another page (`/…`) to the site's. Written to build/client/llms.txt at build time.
const PAGES = [
  { files: pageFiles(), url: '/' },
  { files: ['postgres.mdx'], url: '/postgres' },
  { files: ['mysql.mdx'], url: '/mysql' },
  { files: ['sqlite.mdx'], url: '/sqlite' },
];

const split = (text: string) => text.match(/^---\n([\s\S]*?)\n---\n([\s\S]*)$/) ?? [];

async function markdown({ files, url }: (typeof PAGES)[number]): Promise<string> {
  const [first, ...rest] = await Promise.all(files.map((file) => readFile(path.resolve('content/docs', file), 'utf8')));
  const [, frontmatter = '', intro = first!] = split(first!);
  const field = (name: string) => frontmatter.match(new RegExp(`^${name}: (.*)$`, 'm'))?.[1] ?? '';
  const body = [intro, ...rest.map((text) => split(text)[2] ?? text)].join('\n');
  const page = url === '/' ? `${siteUrl}/` : `${siteUrl}${url}`;
  const text = toAgentMarkdown(body).replaceAll('](#', `](${page}#`).replaceAll('](/', `](${siteUrl}/`);
  return `# ${field('title')}\n\n> ${field('description')}\n\n${text.trim()}\n`;
}

export async function loader() {
  const text = (await Promise.all(PAGES.map(markdown))).join('\n');
  return new Response(text, { headers: { 'Content-Type': 'text/markdown; charset=utf-8' } });
}
