'use client';

import { ThemeSwitch } from 'fumadocs-ui/layouts/shared/slots/theme-switch';
import { Mark } from '@/components/mark';
import { githubUrl } from '@/lib/layout.shared';

/**
 * The one bar above each page: the mark and the name, the Details page, the repository and the theme switch. Fumadocs' own docs
 * header shows only on small screens, where it opens the sidebar tree; this page has no tree, so
 * the bar is the same at every width, full width, in the place of a banner.
 */
export function Header() {
  return (
    <header
      className="sticky top-0 z-40 flex h-(--fd-banner-height) items-center gap-4 border-b bg-fd-background/85 px-4 backdrop-blur-sm md:px-6 xl:px-8"
    >
      <a href="/" className="inline-flex items-center gap-2 font-semibold tracking-tight text-fd-headings">
        <Mark className="h-3.5 w-auto" />
        Reylo
      </a>
      <div className="flex-1" />
      <a href="/details" className="text-sm text-fd-muted-foreground transition-colors hover:text-fd-foreground">
        Details
      </a>
      <a
        href={githubUrl}
        rel="noreferrer noopener"
        target="_blank"
        className="text-sm text-fd-muted-foreground transition-colors hover:text-fd-foreground"
      >
        GitHub
      </a>
      <ThemeSwitch />
    </header>
  );
}
