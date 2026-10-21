import { existsSync, readdirSync } from 'node:fs';
import path from 'node:path';

/**
 * The main page's files, in the order it shows them: content/docs/index.mdx, which holds the title,
 * the opening and the schema, then one file per h2 section, content/sections/*.mdx, by name. A
 * section is a file of its own so that an edit compiles that section alone (wiki/docs-site.md).
 */
export function pageFiles(site = process.cwd()): string[] {
  const sections = path.join(site, 'content', 'sections');
  const names = existsSync(sections) ? readdirSync(sections).filter((name) => name.endsWith('.mdx')).sort() : [];
  return [path.join(site, 'content', 'docs', 'index.mdx'), ...names.map((name) => path.join(sections, name))];
}

/** Every page of the site as its files: the main page's (`pageFiles`), then each other file of content/docs, a page by itself, by name. */
export function sitePages(site = process.cwd()): string[][] {
  const docs = path.join(site, 'content', 'docs');
  const others = readdirSync(docs).filter((name) => name.endsWith('.mdx') && name !== 'index.mdx').sort();
  return [pageFiles(site), ...others.map((name) => [path.join(docs, name)])];
}
