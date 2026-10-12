import { source } from '@/lib/source';
import { Header } from '@/components/header';
import { getMDXComponents } from '@/components/mdx';
import { DocsLayout } from 'fumadocs-ui/layouts/docs';
import { DocsBody, DocsDescription, DocsPage, DocsTitle } from 'fumadocs-ui/layouts/docs/page';
import type { Metadata } from 'next';
import { notFound } from 'next/navigation';

type Props = { params: Promise<{ slug?: string[] }> };

// Two pages: content/docs/index.mdx at / and content/docs/details.mdx at /details, each with its
// table of contents and no sidebar tree.
export default async function Page({ params }: Props) {
  const page = source.getPage((await params).slug ?? []);
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

export function generateStaticParams() {
  return source.generateParams();
}

export async function generateMetadata({ params }: Props): Promise<Metadata> {
  const page = source.getPage((await params).slug ?? []);
  if (!page) notFound();
  return { title: page.slugs.length ? `${page.data.title} · Reylo` : page.data.title, description: page.data.description };
}
