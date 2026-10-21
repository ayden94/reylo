import './global.css';
import { DocsRoot } from 'docs-kit/root';
import { Caveat, Geist, Geist_Mono } from 'next/font/google';
import type { Metadata } from 'next';
import type { ReactNode } from 'react';
import { DIALECTS } from '@/lib/dialects';
import { siteTitle, siteUrl } from '@/lib/layout.shared';
import { source } from '@/lib/source';

// Geist for text and Geist Mono for code: the review of 2026-10-20 ("too wide", "the legibility
// isn't very good anymore") replaced Inter and JetBrains Mono, after a comparison of six
// directions on the live page. Both are variable fonts, so they take no weight list.
//
// `display: 'optional'`, not 'swap': a font that arrived after the first paint rewrapped the
// opening paragraph (a layout shift of 0.057 at 1440 wide and 0.146 at 390, measured 2026-10-20).
// The files are preloaded, so they make the first paint on any ordinary connection; a first view
// whose fonts take longer keeps the fallbacks, and the next view has them cached. Geist Mono's
// fallback is the system's monospace, not the Arial next/font would adjust to it, which drew code
// proportional until the font came; under Turbopack `adjustFontFallback: false` alone kept the
// Arial face, so the fallback is listed.
const sans = Geist({
  subsets: ['latin'],
  display: 'optional',
  variable: '--font-text',
});

const mono = Geist_Mono({
  subsets: ['latin'],
  display: 'optional',
  adjustFontFallback: false,
  fallback: ['ui-monospace', 'SF Mono', 'Menlo', 'monospace'],
  variable: '--font-code',
});

// A handwriting face, as on nub's and Motherload's docs, for the "Leave a star!" note under the
// GitHub link only (docs-kit's chrome).
const hand = Caveat({
  subsets: ['latin'],
  weight: ['600'],
  display: 'swap',
  variable: '--font-hand',
});

// The site is one page, so its metadata is the page's: the title, the frontmatter's description,
// and the link previews, whose image is app/opengraph-image.tsx. It lives here rather than on the
// page because Next.js replaces an inherited `openGraph` or `alternates` whole: a page that set its
// own would drop the image Next.js attaches at this level. The icons are linked in <head> below.
export function generateMetadata(): Metadata {
  const description = source.getPage([])?.data.description;
  return {
    metadataBase: new URL(siteUrl),
    title: siteTitle,
    description,
    applicationName: 'Reylo',
    authors: [{ name: 'Colin McDonnell', url: 'https://github.com/colinhacks' }],
    creator: 'Colin McDonnell',
    robots: { index: true, follow: true, googleBot: { index: true, follow: true, 'max-image-preview': 'large', 'max-snippet': -1 } },
    alternates: { canonical: '/', types: { 'text/markdown': '/llms.txt' } },
    openGraph: { type: 'website', url: '/', siteName: 'Reylo', locale: 'en_US', title: siteTitle, description },
    twitter: { card: 'summary_large_image', creator: '@colinhacks', title: siteTitle, description },
  };
}

// The favicon follows the browser's theme: the black mark on a light tab, the white one on a dark
// tab. The SVG's own media query (public/icon.svg) does this in Chrome and Firefox, but Safari
// ignores it (WebKit bug 309949), and Chrome draws the icon once and keeps it when the theme
// changes. So this script adds the icon links itself, each naming its file for the theme (the ICO
// for browsers that read no SVG icon), and changes them when the theme changes, which makes Chrome
// read them again. It runs while <head> is parsed, before Safari reads the page's icons, which it
// does once, just before the load event, so Safari shows the icon of the theme the page loaded in.
// React never sees these links: React 19 matches the icon links it renders by their href, and puts
// back the original of any whose href a script has changed. The ICO is declared 48 pixels and the
// SVG any size, so Chrome takes the SVG (it scores `any` above a size it would have to scale). The
// icons follow the browser, not the site's theme switch, since the browser draws the tab.
const themeIcons = `try {
  const dark = matchMedia('(prefers-color-scheme: dark)');
  const icon = (sizes, type) => {
    const link = document.createElement('link');
    link.rel = 'icon';
    link.sizes = sizes;
    if (type) link.type = type;
    document.head.appendChild(link);
    return link;
  };
  const ico = icon('48x48');
  const svg = icon('any', 'image/svg+xml');
  const apply = () => {
    ico.href = dark.matches ? '/favicon-dark.ico' : '/favicon.ico';
    svg.href = dark.matches ? '/icon-dark.svg' : '/icon-light.svg';
  };
  apply();
  dark.addEventListener('change', apply);
} catch {}`;

// For a client that runs no script, such as a crawler: the black ICO and the SVG that colours
// itself by its own media query.
const staticIcons = '<link rel="icon" href="/favicon.ico" sizes="48x48"><link rel="icon" href="/icon.svg" type="image/svg+xml" sizes="any">';

// docs-kit's document: Fumadocs' provider, and the head scripts that show a saved tab choice from
// the first paint and keep a pressed tab under the pointer. Every dialect choice on the page is one
// tab group, `dialect` (components/dialect-tabs.tsx); a saved choice written in another case, such
// as the `postgres` an earlier version of the page saved, is put in today's spelling before the tabs
// read it, and any other value is dropped, since Fumadocs' tabs would otherwise select nothing.
export default function Layout({ children }: { children: ReactNode }) {
  return (
    <DocsRoot
      className={`${sans.variable} ${mono.variable} ${hand.variable}`}
      tabs={{ dialect: DIALECTS }}
      head={
        <>
          <script dangerouslySetInnerHTML={{ __html: themeIcons }} />
          <noscript dangerouslySetInnerHTML={{ __html: staticIcons }} />
        </>
      }
    >
      {children}
    </DocsRoot>
  );
}
