import { source } from '@/lib/source';
import { githubUrl, pageTitle, siteUrl } from '@/lib/layout.shared';
import { Brand } from '@/components/brand';
import { getMDXComponents } from '@/components/mdx';
import { CopyAgentPrompt } from 'docs-kit/components';
import { DocsShell } from 'docs-kit/shell';
import type { Metadata, ResolvingMetadata } from 'next';
import { notFound } from 'next/navigation';

type Props = { params: Promise<{ slug?: string[] }> };

/**
 * The h2s whose h3s stay out of each page's table of contents. The maintainer wants the whole table
 * to fit on one screen (a window 900px tall, at Fumadocs' spacing); the headings stay on the page,
 * with their anchors. On the main page: the query's members under Usage, the parameters, the keys
 * of `.reylo.yaml` and the examples. On each dialect page (content/docs/<dialect>.mdx, one skeleton
 * for all three): the h3s of Parameters, which leaves Data types' h3s, the reason a reader lands
 * there, in the table.
 */
const FOLD: Record<string, readonly string[]> = {
  '': ['#usage', '#parameters', '#configuration', '#examples'],
  postgres: ['#parameters'],
  mysql: ['#parameters'],
  sqlite: ['#parameters'],
};

export default async function Page({ params }: Props) {
  const slug = (await params).slug ?? [];
  const page = source.getPage(slug);
  if (!page) notFound();
  const MDX = page.data.body;
  const main = slug.length === 0;

  // docs-kit's page (docs-kit/src/components/shell.tsx): the table of contents on the left, with
  // the brand at its top (a link to the main page) and the GitHub link, the "Leave a star!" note
  // and the theme switch at its foot. The subheader under the title is the page's first element
  // (`.dk-subheader`), since it may set code; the frontmatter's description, in plain text, is the
  // page's metadata (app/layout.tsx, and generateMetadata below for every other page). On the main
  // page only, "Copy agent prompt" at the right end of the title's row copies the agent card at the
  // top of Setup (docs-kit's CopyAgentPrompt finds it in the page, and no other page has one), and
  // the structured data describes the site and the project.
  return (
    <DocsShell
      tree={source.getPageTree()}
      title={page.data.title}
      toc={page.data.toc}
      fold={FOLD[slug.join('/')] ?? []}
      brand={<Brand />}
      github={githubUrl}
      titleAside={main ? <CopyAgentPrompt /> : undefined}
      before={main ? <script type="application/ld+json" dangerouslySetInnerHTML={{ __html: structuredData(page.data.description ?? '') }} /> : undefined}
    >
      <MDX components={getMDXComponents()} />
    </DocsShell>
  );
}

export function generateStaticParams() {
  return source.generateParams();
}

/**
 * The main page's metadata is the layout's (app/layout.tsx). Every other page, a dialect's
 * reference, has its own title, description and address, in the link previews too. Next.js
 * replaces an inherited `openGraph` or `twitter` whole, so each keeps the image the layout's
 * segment carries (app/opengraph-image.tsx), read from the parent's resolved metadata.
 */
export async function generateMetadata({ params }: Props, parent: ResolvingMetadata): Promise<Metadata> {
  const slug = (await params).slug ?? [];
  const page = source.getPage(slug);
  if (!page || slug.length === 0) return {};
  const inherited = await parent;
  const title = pageTitle(page.data.title);
  const description = page.data.description;
  return {
    title,
    description,
    alternates: { canonical: page.url, types: { 'text/markdown': '/llms.txt' } },
    openGraph: { type: 'article', url: page.url, siteName: 'Reylo', locale: 'en_US', title, description, images: inherited.openGraph?.images },
    twitter: { card: 'summary_large_image', creator: '@colinhacks', title, description, images: inherited.twitter?.images },
  };
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
