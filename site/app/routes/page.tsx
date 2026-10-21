import { createHash } from 'node:crypto';
import { readFile } from 'node:fs/promises';
import { use } from 'react';
import { CopyAgentPrompt } from 'docs-kit/components';
import { DocsShell } from 'docs-kit/shell';
import type { Root } from 'fumadocs-core/page-tree';
import type { Route } from './+types/page';
import { Brand } from '@/components/brand';
import { getMDXComponents } from '@/components/mdx';
import { loadPage } from '@/lib/content';
import { githubUrl, pageTitle, siteTitle, siteUrl } from '@/lib/layout.shared';
import { ogAlt, ogSize } from '@/lib/og';
import { source } from '@/lib/source';

/**
 * The h2s whose h3s stay out of each page's table of contents. The maintainer wants the whole table
 * to fit on one screen (a window 900px tall, at Fumadocs' spacing); the headings stay on the page,
 * with their anchors. On the main page: the examples (Parameters' h3s show since 2026-10-21, as the maintainer asked); the query's members
 * under Usage and the keys of `.reylo.yaml` show, as the maintainer asked on 2026-10-20 ("All the h3s
 * underneath the usage section should show up in the sidebar"; "Same for configuration"). On each dialect page (content/docs/<dialect>.mdx, one skeleton
 * for all three): the h3s of Parameters, which leaves Data types' h3s, the reason a reader lands
 * there, in the table.
 */
const FOLD: Record<string, readonly string[]> = {
  '': ['#examples'],
  postgres: ['#parameters'],
  mysql: ['#parameters'],
  sqlite: ['#parameters'],
};

/**
 * Run at build time only, when each page is rendered to a file (react-router.config.ts): the page
 * as the meta tags and the shell need it, without its content, which the component loads itself
 * (lib/content.ts). The link preview's URL carries a hash of the text of the image's route, so that
 * a change to the card changes the URL and the caches of X and Slack fetch the new card.
 */
export async function loader({ params }: Route.LoaderArgs) {
  const slug = params.slug ?? '';
  const page = source.getPage(slug ? [slug] : []);
  if (!page) throw new Response('Not found', { status: 404 });
  const card = await readFile('app/routes/opengraph-image.tsx');
  return {
    slug,
    path: page.path,
    url: page.url,
    title: page.data.title,
    description: page.data.description ?? '',
    tree: source.getPageTree(),
    og: createHash('sha256').update(card).digest('hex').slice(0, 16),
  };
}

/**
 * The main page's metadata: the title, the frontmatter's description, and the link previews, whose
 * image is app/routes/opengraph-image.tsx. Each dialect's page has its own title, description and
 * address, in the link previews too, and the same image.
 */
export function meta({ loaderData }: Route.MetaArgs): Route.MetaDescriptors {
  if (!loaderData) return [];
  const main = loaderData.slug === '';
  const title = main ? siteTitle : pageTitle(loaderData.title);
  const { description } = loaderData;
  const url = main ? siteUrl : `${siteUrl}${loaderData.url}`;
  const image = `${siteUrl}/opengraph-image.png?${loaderData.og}`;
  return [
    { title },
    { name: 'description', content: description },
    { name: 'application-name', content: 'Reylo' },
    { name: 'author', content: 'Colin McDonnell' },
    { name: 'creator', content: 'Colin McDonnell' },
    { name: 'robots', content: 'index, follow' },
    { name: 'googlebot', content: 'index, follow, max-image-preview:large, max-snippet:-1' },
    { tagName: 'link', rel: 'canonical', href: url },
    { tagName: 'link', rel: 'alternate', type: 'text/markdown', href: `${siteUrl}/llms.txt` },
    { property: 'og:title', content: title },
    { property: 'og:description', content: description },
    { property: 'og:url', content: url },
    { property: 'og:site_name', content: 'Reylo' },
    { property: 'og:locale', content: 'en_US' },
    { property: 'og:image', content: image },
    { property: 'og:image:type', content: 'image/png' },
    { property: 'og:image:width', content: String(ogSize.width) },
    { property: 'og:image:height', content: String(ogSize.height) },
    { property: 'og:image:alt', content: ogAlt },
    { property: 'og:type', content: main ? 'website' : 'article' },
    { name: 'twitter:card', content: 'summary_large_image' },
    { name: 'twitter:creator', content: '@colinhacks' },
    { name: 'twitter:title', content: title },
    { name: 'twitter:description', content: description },
    { name: 'twitter:image', content: image },
    { name: 'twitter:image:alt', content: ogAlt },
    { name: 'twitter:image:type', content: 'image/png' },
    { name: 'twitter:image:width', content: String(ogSize.width) },
    { name: 'twitter:image:height', content: String(ogSize.height) },
  ];
}

export default function Page({ loaderData }: Route.ComponentProps) {
  const { slug, path, title, description, tree } = loaderData;
  const files = use(loadPage(path));
  const components = getMDXComponents();
  const main = slug === '';

  // docs-kit's page (docs-kit/src/components/shell.tsx): the table of contents on the left, with
  // the brand at its top (a link to the main page) and the GitHub link, the "Leave a star!" note
  // and the theme switch at its foot. The main page is content/docs/index.mdx and then each section
  // of content/sections/, each a module of its own, and its table of contents is theirs in order.
  // The subheader under the title is the page's first element (`.dk-subheader`), since it may set
  // code; the frontmatter's description, in plain text, is the page's metadata (above). On the main
  // page only, "Copy agent prompt" at the right end of the title's row copies the agent card at
  // the top of Setup (docs-kit's CopyAgentPrompt finds it in the page, and no other page has one),
  // and the structured data describes the site and the project. Every page's column ends on the licence.
  return (
    <DocsShell
      // The tree's names are the pages' titles, plain strings, so it reaches the browser as it is
      // (Fumadocs' serializer would wrap each name in a span of its own).
      tree={tree as Root}
      title={title}
      toc={files.flatMap((file) => file.toc)}
      fold={FOLD[slug] ?? []}
      brand={<Brand />}
      github={githubUrl}
      titleAside={main ? <CopyAgentPrompt /> : undefined}
      before={main ? <script type="application/ld+json" dangerouslySetInnerHTML={{ __html: structuredData(description) }} /> : undefined}
      footer={<>Released under the MIT License. Copyright © 2026 Colin McDonnell.</>}
    >
      {files.map((file, i) => (
        <file.default key={i} components={components} />
      ))}
    </DocsShell>
  );
}

/**
 * The site and the project as schema.org data: the WebSite gives search results the site's name,
 * and the SoftwareSourceCode says what the page is about and where its code lives.
 */
function structuredData(description: string): string {
  const author = { '@type': 'Person', name: 'Colin McDonnell', url: 'https://github.com/colinhacks' };
  const data = {
    '@context': 'https://schema.org',
    '@graph': [
      { '@type': 'WebSite', '@id': `${siteUrl}/#website`, url: `${siteUrl}/`, name: 'Reylo', description, inLanguage: 'en' },
      {
        '@type': 'SoftwareSourceCode',
        '@id': `${siteUrl}/#software`,
        name: 'Reylo',
        description,
        url: `${siteUrl}/`,
        codeRepository: githubUrl,
        programmingLanguage: ['TypeScript', 'SQL'],
        runtimePlatform: 'Node.js',
        license: 'https://opensource.org/licenses/MIT',
        author,
        isPartOf: { '@id': `${siteUrl}/#website` },
      },
    ],
  };
  // `<` is escaped so that no string in the data can close the script element.
  return JSON.stringify(data).replace(/</g, '\\u003c');
}
