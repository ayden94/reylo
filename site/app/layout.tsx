import './global.css';
import { RootProvider } from 'fumadocs-ui/provider/next';
import { Caveat, Geist, Geist_Mono } from 'next/font/google';
import type { ReactNode } from 'react';

// Geist for text and Geist Mono for code: the review of 2026-10-20 ("too wide", "the legibility
// isn't very good anymore") replaced Inter and JetBrains Mono, after a comparison of six
// directions on the live page. Both are variable fonts, so they take no weight list.
const sans = Geist({
  subsets: ['latin'],
  display: 'swap',
  variable: '--font-text',
});

const mono = Geist_Mono({
  subsets: ['latin'],
  display: 'swap',
  variable: '--font-code',
});

// A handwriting face, as on nub's and Motherload's docs, for the "Leave a star!" note under the
// GitHub link only (components/chrome.tsx).
const hand = Caveat({
  subsets: ['latin'],
  weight: ['600'],
  display: 'swap',
  variable: '--font-hand',
});

export const metadata = {
  title: 'Reylo',
  description: 'Import a .sql file and get a fully typed query, checked against the real database.',
};

// Fumadocs' tabs take the dialect saved under `dialect` (components/dialect-tabs.tsx) over their
// default, and a saved value that names no tab selects nothing, so every dialect group showed an
// empty panel until a tab was clicked. An earlier version of the page saved `postgres`, `mysql` or
// `sqlite` (`<Tabs items>` lowercases its values). This runs before the tabs read the choice: it
// keeps a saved dialect that differs only in case, in today's spelling (DIALECTS there), and drops
// any other value, so the tabs fall back to their default.
const restoreDialect = `try {
  for (const store of [sessionStorage, localStorage]) {
    const saved = store.getItem('dialect');
    if (saved === null) continue;
    const dialect = ['Postgres', 'MySQL', 'SQLite'].find((d) => d.toLowerCase() === saved.toLowerCase());
    if (dialect) store.setItem('dialect', dialect);
    else store.removeItem('dialect');
  }
} catch {}`;

export default function Layout({ children }: { children: ReactNode }) {
  return (
    <html lang="en" suppressHydrationWarning className={`${sans.variable} ${mono.variable} ${hand.variable}`}>
      <head>
        <script dangerouslySetInnerHTML={{ __html: restoreDialect }} />
      </head>
      <body className="flex flex-col min-h-screen">
        <RootProvider search={{ enabled: false }}>{children}</RootProvider>
      </body>
    </html>
  );
}
