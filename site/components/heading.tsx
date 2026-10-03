'use client';

import { useState, type ComponentPropsWithoutRef } from 'react';

type Props = ComponentPropsWithoutRef<'h2'> & { as: 'h2' | 'h3' | 'h4' };

/**
 * A section heading with a link icon in the margin to its left, always shown in a light gray.
 * Clicking it copies the section's URL and puts its anchor in the address bar. The heading text
 * is plain text, not a link, so it carries none of the link underline.
 */
export function Heading({ as: As, id, children, className, ...props }: Props) {
  const [copied, setCopied] = useState(false);
  if (!id) return <As className={className} {...props}>{children}</As>;
  const copy = () => {
    const url = new URL(window.location.href);
    url.hash = id;
    history.replaceState(null, '', url);
    void navigator.clipboard.writeText(url.href).then(() => {
      setCopied(true);
      setTimeout(() => setCopied(false), 1500);
    });
  };
  return (
    <As id={id} className={['motherload-heading', className].filter(Boolean).join(' ')} {...props}>
      <button type="button" className="motherload-anchor not-prose" aria-label="Copy a link to this section" onClick={copy}>
        <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden>
          {copied ? (
            <path d="M20 6 9 17l-5-5" />
          ) : (
            <>
              <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" />
              <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" />
            </>
          )}
        </svg>
      </button>
      {children}
    </As>
  );
}
