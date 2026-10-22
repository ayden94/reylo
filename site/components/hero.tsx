import { CopyAgentPrompt } from 'docs-kit/components';
import type { ReactNode } from 'react';
import { wordmark } from '@/components/wordmark';
import { githubUrl } from '@/lib/layout.shared';

/**
 * The main page's title (app/routes/page.tsx): the mark over the name, centred, the name in
 * Orbitron, the face of the wordmark (app/global.css, `.hero-title`). The mark is decoration; the
 * heading reads as the name.
 */
export function HeroTitle({ children }: { children: ReactNode }) {
  return (
    <span className="hero-title">
      <svg viewBox="0 0 400 235.47" fill="currentColor" aria-hidden="true" className="hero-mark">
        <defs>
          {/* The bloom: the mark's shape blurred twice, near and far, lit gold and amber (app/global.css, .hero-bloom-*). */}
          <filter id="hero-bloom" x="-40%" y="-60%" width="180%" height="220%" colorInterpolationFilters="sRGB">
            <feGaussianBlur in="SourceAlpha" stdDeviation="8" result="near" />
            <feFlood className="hero-bloom-near" />
            <feComposite in2="near" operator="in" result="nearLight" />
            <feGaussianBlur in="SourceAlpha" stdDeviation="22" result="far" />
            <feFlood className="hero-bloom-far" />
            <feComposite in2="far" operator="in" result="farLight" />
            <feMerge>
              <feMergeNode in="farLight" />
              <feMergeNode in="nearLight" />
              <feMergeNode in="SourceGraphic" />
            </feMerge>
          </filter>
        </defs>
        <path d={wordmark.mark} filter="url(#hero-bloom)" />
      </svg>
      {children}
    </span>
  );
}

/**
 * Under the main page's subheader (content/docs/index.mdx): the licence, npm and GitHub stars
 * badges, then "Copy agent prompt", centred, as at the top of Zod's README.
 */
export function HeroActions() {
  return (
    <div className="hero-actions not-prose">
      <p className="hero-badges">
        <a href="https://opensource.org/licenses/MIT">
          <img src="https://img.shields.io/github/license/colinhacks/reylo" alt="License: MIT" height={20} />
        </a>
        <a href="https://www.npmjs.com/package/reylo">
          <img src="https://img.shields.io/npm/v/reylo" alt="npm version" height={20} />
        </a>
        <a href={githubUrl}>
          <img src="https://img.shields.io/github/stars/colinhacks/reylo" alt="GitHub stars" height={20} />
        </a>
      </p>
      <CopyAgentPrompt />
    </div>
  );
}
