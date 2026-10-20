// The link preview reylo.dev shows on X, Slack, Discord, LinkedIn and the rest: Next.js links this
// image from every page as og:image, and as X's image too, with a hash of this file's text in its
// URL. The card is drawn here rather than in a module this file imports, so that a change to the
// card changes the URL and the caches of X and Slack fetch the new card.
//
// The whole card is a code block, drawn as the page draws its opening example: a tab strip naming
// `main.ts` and `get_author.sql`, the subheader's claim as a comment, the import of a `.sql` file
// and its `run`, and the type of `author` in a popup at the code's own size, as the page shows it
// on hover. The wordmark and the address sit in gaps the code leaves: the wordmark at the end of
// the tab strip, the address beside the popup's last line. The maintainer asked for that on
// 2026-10-20 ("the entire card is the code block, but then you're able to put the logo, the website
// link, and things like that into the natural gaps within the code"); the card before it set the
// claim as a headline over two lines of code, with the type on one line in smaller text.
//
// Drawn by Next.js's ImageResponse (Satori) in the site's dark theme: Geist Mono (assets/fonts, SIL
// Open Font License) on the dark code block's colours, the code in the page's GitHub Dark colours
// with its brighter comment grey (docs-kit/src/style.css). Geist Mono's characters are 0.6 em wide,
// so the popup is placed by character. Satori lays out flex boxes only, so every element with more
// than one child is a flex box.
import { readFile } from 'node:fs/promises';
import { join } from 'node:path';
import { ImageResponse } from 'next/og';
import type { ReactNode } from 'react';
import { wordmark } from '@/components/wordmark';

export const size = { width: 1200, height: 630 };
export const contentType = 'image/png';

export const alt =
  'Reylo: a code editor showing main.ts. A comment reads "Import .sql files into TypeScript, fully typed", the code imports getAuthor from "./get_author.sql" and runs it, and a popup gives the type of the result: { id: number; name: string; post_count: number } | null.';

// The dark theme of app/global.css, and Shiki's GitHub Dark for the code.
const color = {
  surface: '#161616',
  popup: '#1d1d1d',
  border: '#2b2b2b',
  headings: '#fafafa',
  muted: '#a1a1a1',
  yellow: '#fbd22d',
  keyword: '#f97583',
  string: '#9ecbff',
  function: '#b392f0',
  constant: '#79b8ff',
  plain: '#e1e4e8',
  property: '#ffab70',
  comment: '#8b949e',
};

type Token = [text: string, color: keyof typeof color, hovered?: true];

const code: Token[][] = [
  [['// Import .sql files into TypeScript, fully typed', 'comment']],
  [['import', 'keyword'], [' getAuthor ', 'plain'], ['from', 'keyword'], [' "./get_author.sql"', 'string'], [';', 'plain']],
  [],
  [['const', 'keyword'], [' ', 'plain'], ['author', 'constant', true], [' = ', 'keyword'], ['await', 'keyword'], [' getAuthor.', 'plain'], ['run', 'function'], ['(db, { id: ', 'plain'], ['1', 'constant'], [' });', 'plain']],
];

// The type the page's popup shows for `author` (site/lib/sql-modules.json), laid out as the page lays it out.
const authorType: Token[][] = [
  [['const', 'keyword'], [' author', 'constant'], [': {', 'plain']],
  [['    id', 'property'], [': ', 'plain'], ['number', 'constant'], [';', 'plain']],
  [['    name', 'property'], [': ', 'plain'], ['string', 'constant'], [';', 'plain']],
  [['    post_count', 'property'], [': ', 'plain'], ['number', 'constant'], [';', 'plain']],
  [['} | ', 'plain'], ['null', 'constant']],
];

// Sizes, in pixels of the 1200 × 630 image.
const SIZE = 30;
const ROW = SIZE * 1.6;
const CHAR = SIZE * 0.6;
const PAD = 64;
const TABS = 78;
const TOP = 42;
const MARK = 40;
const HOVERED = 3; // the line of `author`
const POPUP_Y = 12;

function Lines({ lines }: { lines: Token[][] }) {
  return (
    <div style={{ display: 'flex', flexDirection: 'column', fontFamily: 'Geist Mono', fontSize: SIZE }}>
      {lines.map((line, i) => (
        <div key={i} style={{ display: 'flex', whiteSpace: 'pre', height: ROW, alignItems: 'center' }}>
          {line.map(([text, ink, hovered], j) => (
            <span key={j} style={{ color: color[ink], ...(hovered ? { borderBottom: `3px solid ${color.yellow}`, paddingBottom: 1 } : {}) }}>
              {text}
            </span>
          ))}
        </div>
      ))}
    </div>
  );
}

function Card(): ReactNode {
  const popupTop = TABS + TOP + ROW * (HOVERED + 1) + 6;
  return (
    <div style={{ width: '100%', height: '100%', display: 'flex', flexDirection: 'column', position: 'relative', background: color.surface, fontFamily: 'Geist Mono' }}>
      <div style={{ display: 'flex', alignItems: 'flex-end', height: TABS, padding: `0 ${PAD}px`, borderBottom: `1.5px solid ${color.border}`, fontSize: 24 }}>
        <div style={{ display: 'flex', color: color.yellow, borderBottom: `3px solid ${color.yellow}`, paddingBottom: 16, marginRight: 36 }}>main.ts</div>
        <div style={{ display: 'flex', color: color.muted, paddingBottom: 19 }}>get_author.sql</div>
        <div style={{ display: 'flex', flex: 1 }} />
        <div style={{ display: 'flex', paddingBottom: (TABS - MARK) / 2 - 2 }}>
          <svg width={(MARK * wordmark.width) / wordmark.height} height={MARK} viewBox={wordmark.viewBox}>
            <path fill={color.headings} d={wordmark.mark} />
            <path fill={color.headings} transform={wordmark.wordTransform} d={wordmark.word} />
          </svg>
        </div>
      </div>
      <div style={{ display: 'flex', padding: `${TOP}px ${PAD}px 0` }}>
        <Lines lines={code} />
      </div>
      {/* Under `author`, which starts six characters into its line, as the page's popup opens. */}
      <div
        style={{
          position: 'absolute',
          left: PAD + CHAR * 6 - 24,
          top: popupTop,
          display: 'flex',
          padding: `${POPUP_Y}px 24px`,
          background: color.popup,
          border: `1.5px solid ${color.border}`,
          borderRadius: 14,
          boxShadow: '0 16px 48px rgba(0, 0, 0, 0.5)',
        }}
      >
        <Lines lines={authorType} />
      </div>
      <div style={{ position: 'absolute', right: PAD, top: popupTop + POPUP_Y + ROW * 4, height: ROW, display: 'flex', alignItems: 'center', fontSize: 24, color: color.muted }}>
        reylo.dev
      </div>
    </div>
  );
}

/** The preview as a PNG, built once, with the site. */
export default async function Image() {
  const mono = await readFile(join(process.cwd(), 'assets/fonts/GeistMono-400.ttf'));
  return new ImageResponse(<Card />, { ...size, fonts: [{ name: 'Geist Mono', weight: 400, style: 'normal', data: mono }] });
}
