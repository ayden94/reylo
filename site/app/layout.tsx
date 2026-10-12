import './global.css';
import { RootProvider } from 'fumadocs-ui/provider/next';
import { Caveat, Inter, JetBrains_Mono } from 'next/font/google';
import type { ReactNode } from 'react';

// Inter for text, as on zod.dev; JetBrains Mono for code.
const sans = Inter({
  subsets: ['latin'],
  display: 'swap',
  variable: '--font-text',
});

const mono = JetBrains_Mono({
  subsets: ['latin'],
  weight: ['400', '500'],
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

export default function Layout({ children }: { children: ReactNode }) {
  return (
    <html lang="en" suppressHydrationWarning className={`${sans.variable} ${mono.variable} ${hand.variable}`}>
      <body className="flex flex-col min-h-screen">
        <RootProvider search={{ enabled: false }}>{children}</RootProvider>
      </body>
    </html>
  );
}
