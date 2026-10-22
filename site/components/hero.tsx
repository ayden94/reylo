import { CopyAgentPrompt } from 'docs-kit/components';
import { useEffect, useRef, type ReactNode } from 'react';
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
        <path d={wordmark.mark} />
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

// The demo's markup as HTML: React leaves `muted` out of server-rendered HTML, and a browser
// autoplays only a muted video, so a <video> rendered by React would sit still until hydration.
// A phone takes the portrait cut (its media query matches app/global.css's breakpoint).
const demoVideo =
  '<video autoplay loop muted playsinline preload="auto" aria-label="An editor: hovering user in main.ts shows its type; the SQL in get_user.sql gains columns and loses its where clause, and the type follows">' +
  '<source src="/demo/reylo-phone.mp4" type="video/mp4" media="(max-width: 767px)">' +
  '<source src="/demo/reylo.mp4" type="video/mp4">' +
  '</video>';

/**
 * Under the badges and "Copy agent prompt": a looping recording of an editor, the type of a query's
 * result following its SQL as the SQL changes. Its box keeps the video's shape from the first paint
 * (app/global.css, `.hero-demo`), so nothing moves when the video loads; a reader who asks for
 * reduced motion gets its first frame, still.
 */
export function HeroDemo() {
  const box = useRef<HTMLDivElement>(null);
  useEffect(() => {
    const video = box.current?.querySelector('video');
    const reduce = window.matchMedia('(prefers-reduced-motion: reduce)');
    if (!video) return;
    const apply = () => {
      if (reduce.matches) {
        video.pause();
        video.currentTime = 0;
      } else void video.play().catch(() => {});
    };
    apply();
    reduce.addEventListener('change', apply);
    return () => reduce.removeEventListener('change', apply);
  }, []);
  return <div ref={box} className="hero-demo not-prose" dangerouslySetInnerHTML={{ __html: demoVideo }} />;
}
