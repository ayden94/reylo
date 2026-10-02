import { source } from '@/lib/source';
import { Header } from '@/components/header';
import { getMDXComponents } from '@/components/mdx';
import { DocsLayout } from 'fumadocs-ui/layouts/docs';
import { DocsBody, DocsDescription, DocsPage, DocsTitle } from 'fumadocs-ui/layouts/docs/page';
import type { Metadata } from 'next';
import { notFound } from 'next/navigation';

// The whole site is this one page: content/docs/index.mdx, with its table of contents and no
// sidebar tree.
export default function Page() {
  const page = source.getPage([]);
  if (!page) notFound();
  const MDX = page.data.body;

  return (
    // The bar above the layout takes the place Fumadocs reserves for a banner, so the sticky
    // table of contents starts below it.
    <div className="[--fd-banner-height:--spacing(14)]">
      <Header />
      <DocsLayout tree={source.getPageTree()} sidebar={{ enabled: false }} nav={{ enabled: false }}>
        <DocsPage
          toc={page.data.toc}
          tableOfContent={{ style: 'clerk' }}
          breadcrumb={{ enabled: false }}
          footer={{ enabled: false }}
        >
          <DocsTitle>{page.data.title}</DocsTitle>
          <DocsDescription>{page.data.description}</DocsDescription>
          <DocsBody>
            <MDX components={getMDXComponents()} />
          </DocsBody>
        </DocsPage>
      </DocsLayout>
    </div>
  );
}

export function generateMetadata(): Metadata {
  const page = source.getPage([]);
  if (!page) notFound();
  return { title: page.data.title, description: page.data.description };
}
