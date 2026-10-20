import { source } from '@/lib/source';
import { TocFooter, TocHeader, TopRow } from '@/components/chrome';
import { getMDXComponents } from '@/components/mdx';
import { DocsLayout } from 'fumadocs-ui/layouts/docs';
import { DocsBody, DocsDescription, DocsPage, DocsTitle } from 'fumadocs-ui/layouts/docs/page';
import type { TOCItemType } from 'fumadocs-core/toc';
import type { Metadata } from 'next';
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
  '--fd-layout-width': '78rem',
} as CSSProperties;

/** The h2s whose h3s stay out of the table of contents: the examples, and the keys of `.reylo.yaml`. */
const FOLDED = new Set(['#configuration', '#examples']);

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
        {/* The frontmatter's description is the subheader under the title, "Import SQL files into
            TypeScript.", which the maintainer asked for on 2026-10-20, and the page's metadata
            (below). */}
        <DocsTitle>{page.data.title}</DocsTitle>
        <DocsDescription>{page.data.description}</DocsDescription>
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

export async function generateMetadata({ params }: Props): Promise<Metadata> {
  const page = source.getPage((await params).slug ?? []);
  if (!page) notFound();
  return { title: page.data.title, description: page.data.description };
}
