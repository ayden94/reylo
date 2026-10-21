import { readFile } from 'node:fs/promises';
import path from 'node:path';
import { pageMarkdown } from 'docs-kit/remark-sections';
import { siteUrl } from '@/lib/layout.shared';
import { pageFiles } from '@/lib/page-files';

/**
 * A page of content/docs as Markdown, by its slug ('' for the main page): its files in order (the
 * main page's are lib/page-files.ts'), read as each heading's copy button reads its section, under
 * its title and description, with each link made absolute (docs-kit's pageMarkdown). Each page's
 * is written to build/client/<slug>.md (index.md for the main page) by app/routes/markdown.ts, which
 * vercel.json serves at the page's own address to an agent that asks for Markdown, and
 * /llms.txt is all of them (app/routes/llms.ts).
 */
export async function markdownOf(slug: string): Promise<string> {
  const files = slug ? [path.resolve('content/docs', `${slug}.mdx`)] : pageFiles();
  const sources = await Promise.all(files.map((file) => readFile(file, 'utf8')));
  return pageMarkdown(sources, { site: siteUrl, page: slug ? `/${slug}` : '/' });
}

/** The address of a page's Markdown, by its slug: /index.md for the main page, /<slug>.md for another. */
export const markdownPath = (slug: string) => `/${slug || 'index'}.md`;
