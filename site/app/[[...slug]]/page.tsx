import { source } from '@/lib/source';
import { githubUrl, siteUrl } from '@/lib/layout.shared';
import { Brand } from '@/components/brand';
import { getMDXComponents } from '@/components/mdx';
import { CopyAgentPrompt } from 'docs-kit/components';
import { DocsShell } from 'docs-kit/shell';
import { notFound } from 'next/navigation';

type Props = { params: Promise<{ slug?: string[] }> };

/**
 * The h2s whose h3s stay out of the table of contents: the parameters, the keys of `.reylo.yaml`
 * and the examples. The maintainer wants the whole table to fit on one screen (a window 900px tall,
 * at Fumadocs' spacing); the headings stay on the page, with their anchors.
 */
const FOLD = ['#parameters', '#configuration', '#examples'];

export default async function Page({ params }: Props) {
  const page = source.getPage((await params).slug ?? []);
  if (!page) notFound();
  const MDX = page.data.body;

  // docs-kit's page (docs-kit/src/components/shell.tsx): the table of contents on the left, with
  // the brand at its top and the GitHub link, the "Leave a star!" note and the theme switch at its
  // foot. The subheader under the title is the page's first element (`.dk-subheader`), since it
  // sets `.sql` as code; the frontmatter's description, in plain text, is the page's metadata
  // (app/layout.tsx). "Copy agent prompt" at the right end of the title's row copies the agent
  // card at the top of Setup (docs-kit's CopyAgentPrompt finds it in the page).
  return (
    <DocsShell
      tree={source.getPageTree()}
      title={page.data.title}
      toc={page.data.toc}
      fold={FOLD}
      brand={<Brand />}
      github={githubUrl}
      titleAside={<CopyAgentPrompt />}
      before={<script type="application/ld+json" dangerouslySetInnerHTML={{ __html: structuredData(page.data.description ?? '') }} />}
    >
      <MDX components={getMDXComponents()} />
    </DocsShell>
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
