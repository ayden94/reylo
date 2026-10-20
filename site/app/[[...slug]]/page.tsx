import { source } from '@/lib/source';
import { TocFooter, TocHeader, TopRow } from '@/components/chrome';
import { githubUrl, siteUrl } from '@/lib/layout.shared';
import { getMDXComponents } from '@/components/mdx';
import { DocsLayout } from 'fumadocs-ui/layouts/docs';
import { DocsBody, DocsPage, DocsTitle } from 'fumadocs-ui/layouts/docs/page';
import type { TOCItemType } from 'fumadocs-core/toc';
import type { CSSProperties } from 'react';
import { notFound } from 'next/navigation';

type Props = { params: Promise<{ slug?: string[] }> };

// Fumadocs' docs grid with the table of contents moved to the left, where the sidebar tree would
// be, as on Motherload's docs: the site is one page, so it has no tree. The areas and columns are
// Fumadocs' own (fumadocs-ui/dist/layouts/docs/slots/container.js) with `toc` and `sidebar` swapped.
const LEFT_TOC = {
  gridTemplate: `"toc toc header sidebar sidebar"
"toc toc toc-popover sidebar sidebar"
"toc toc main sidebar sidebar" 1fr / minmax(min-content, 1fr) var(--fd-toc-width) minmax(0, calc(var(--fd-layout-width) - var(--fd-sidebar-width) - var(--fd-toc-width))) var(--fd-sidebar-col) minmax(min-content, 1fr)`,
  // The table of contents (268px, 16.75rem) plus the page: one 40rem column (prose a median 81
  // characters a line, p90 88; code about 77) and Fumadocs' 2rem of padding on each side
  // (`#nd-page` in app/global.css).
  '--fd-layout-width': '60.75rem',
} as CSSProperties;

/** The h2s whose h3s stay out of the table of contents: the parameters, the keys of `.reylo.yaml` and the examples. */
const FOLDED = new Set(['#parameters', '#configuration', '#examples']);

/**
 * The table of contents without the h3s of the sections in FOLDED: the maintainer wants the whole
 * table to fit on one screen (a window 900px tall, at Fumadocs' spacing). The headings stay on the
 * page, with their anchors.
 */
function tableOfContents(toc: TOCItemType[]): TOCItemType[] {
  let folded = false;
  return toc.filter((item) => {
    if (item.depth === 2) folded = FOLDED.has(item.url);
    return !(folded && item.depth > 2);
  });
}

export default async function Page({ params }: Props) {
  const page = source.getPage((await params).slug ?? []);
  if (!page) notFound();
  const MDX = page.data.body;

  return (
    // No bar stays above the page: the name, the repository and the theme switch sit in the left
    // column around the table of contents, or in `TopRow` where Fumadocs hides that column.
    <DocsLayout tree={source.getPageTree()} sidebar={{ enabled: false }} nav={{ enabled: false }} containerProps={{ style: LEFT_TOC }}>
      <TopRow />
      <DocsPage
        toc={tableOfContents(page.data.toc)}
        tableOfContent={{
          style: 'clerk',
          header: <TocHeader />,
          footer: <TocFooter />,
          container: { className: 'justify-self-end ps-4 pt-8' },
        }}
        breadcrumb={{ enabled: false }}
        footer={{ enabled: false }}
      >
        {/* The subheader under the title is the page's first element (`.reylo-subheader`), since it
            sets `.sql` as code; the frontmatter's description, in plain text, is the page's
            metadata (app/layout.tsx). */}
        <script type="application/ld+json" dangerouslySetInnerHTML={{ __html: structuredData(page.data.description ?? '') }} />
        <DocsTitle>{page.data.title}</DocsTitle>
        <DocsBody>
          <MDX components={getMDXComponents()} />
        </DocsBody>
      </DocsPage>
    </DocsLayout>
  );
}

export function generateStaticParams() {
  return source.generateParams();
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
