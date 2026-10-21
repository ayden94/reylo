import type { ReactNode } from 'react';

// The site's theme variables, so the figure follows the light and dark themes as the text does.
const color = {
  text: 'var(--color-fd-foreground)',
  muted: 'var(--color-fd-muted-foreground)',
  headings: 'var(--color-fd-headings)',
  border: 'var(--color-fd-border)',
  card: 'var(--color-fd-card)',
  accent: 'var(--color-fd-primary)',
};
const mono = 'var(--font-code), ui-monospace, monospace';
const sans = 'var(--font-text), ui-sans-serif, system-ui, sans-serif';

const label =
  'How Reylo types a .sql import. TypeScript 7.1 hands get_user.sql to Reylo, its content mapper. Reylo has your database (Postgres, MySQL or SQLite) prepare the statement without running it, reads the catalog, and gives TypeScript the types: for select id, name, email from users where id = :id, the parameters { id: number } and the result { id: number; name: string; email: string | null } | null, at most one row because id is the primary key.';

const nodes = [
  { title: 'TypeScript 7.1', note: 'tsc and the editor' },
  { title: 'Reylo', note: 'content mapper', accent: true },
  { title: 'your database', note: 'Postgres, MySQL or SQLite' },
];

// What goes along each leg between two nodes: the request, then the answer.
const legs = [
  { request: 'get_user.sql', code: true, answer: 'its types' },
  { request: 'prepare, not run', code: false, answer: 'catalog' },
];

const rowCount = 'at most one row: id is the primary key';

type TextProps = { x: number; y: number; children: ReactNode; size?: number; fill?: string; weight?: number; anchor?: 'start' | 'middle' | 'end'; code?: boolean };

/** Real, selectable text in the page's own faces. `pre` keeps the indent of a wrapped type. */
function Text({ x, y, children, size = 12, fill = color.muted, weight = 400, anchor = 'start', code = false }: TextProps) {
  return (
    <text x={x} y={y} textAnchor={anchor} style={{ fill, fontFamily: code ? mono : sans, fontSize: size, fontWeight: weight, whiteSpace: 'pre' }}>
      {children}
    </text>
  );
}

/** A box with the code blocks' thin border and rounded corners; Reylo's in the accent. */
function Box({ x, y, w, h, accent = false }: { x: number; y: number; w: number; h: number; accent?: boolean }) {
  return <rect x={x} y={y} width={w} height={h} rx={12} style={{ fill: color.card, stroke: accent ? color.accent : color.border, strokeWidth: 1 }} />;
}

/** A rule across a box: solid under its title bar, dashed between its statement and its types. */
function Rule({ x, y, w, dashed = false }: { x: number; y: number; w: number; dashed?: boolean }) {
  return <line x1={x} x2={x + w} y1={y} y2={y} style={{ stroke: color.border, strokeDasharray: dashed ? '3 4' : undefined }} />;
}

/** A straight arrow from (x1, y1) to (x2, y2) with an open head, drawn thin and muted. */
function Arrow({ x1, y1, x2, y2 }: { x1: number; y1: number; x2: number; y2: number }) {
  const angle = Math.atan2(y2 - y1, x2 - x1);
  const wing = (turn: number) => `${x2 - 5 * Math.cos(angle + turn)},${y2 - 5 * Math.sin(angle + turn)}`;
  return (
    <g style={{ stroke: color.muted, strokeWidth: 1.25, fill: 'none', strokeLinecap: 'round', strokeLinejoin: 'round' }}>
      <line x1={x1} y1={y1} x2={x2} y2={y2} />
      <polyline points={`${wing(0.6)} ${x2},${y2} ${wing(-0.6)}`} />
    </g>
  );
}

/**
 * A node's name, centred on x with its baseline at y. Reylo's carries the mark before it (the path
 * of components/mark.tsx), as tall as the name's capitals, so that the mark's flat top and bottom
 * sit on the cap line and the baseline; 10.9 is Inter's cap height at 15px.
 */
function NodeTitle({ x, y, node }: { x: number; y: number; node: (typeof nodes)[number] }) {
  if (!node.accent) {
    return <Text x={x} y={y} anchor="middle" size={15} weight={600} fill={color.headings}>{node.title}</Text>;
  }
  const h = 10.9;
  const markWidth = (h * 400) / 235.47;
  const width = markWidth + 7 + 40;
  const left = x - width / 2;
  return (
    <g>
      <path
        transform={`translate(${left} ${y - h}) scale(${h / 235.47})`}
        style={{ fill: color.accent }}
        d="M73.61 0A9.77 9.77 0 0 0 65.16 4.88L2.82 112.85A9.77 9.77 0 0 0 2.82 122.62L65.16 230.59A9.77 9.77 0 0 0 73.61 235.47L107.45 235.47A9.77 9.77 0 0 0 115.91 220.82L59.21 122.62A9.77 9.77 0 0 1 59.21 112.85L115.91 14.65A9.77 9.77 0 0 0 107.45 0ZM284.39 112.85A9.77 9.77 0 0 1 284.39 122.62L246.43 188.38A9.77 9.77 0 0 1 237.97 193.26L162.03 193.26A9.77 9.77 0 0 1 153.57 188.38L115.61 122.62A9.77 9.77 0 0 1 115.61 112.85L153.57 47.09A9.77 9.77 0 0 1 162.03 42.21L237.97 42.21A9.77 9.77 0 0 1 246.43 47.09ZM326.39 0A9.77 9.77 0 0 1 334.84 4.88L397.18 112.85A9.77 9.77 0 0 1 397.18 122.62L334.84 230.59A9.77 9.77 0 0 1 326.39 235.47L292.55 235.47A9.77 9.77 0 0 1 284.09 220.82L340.79 122.62A9.77 9.77 0 0 0 340.79 112.85L284.09 14.65A9.77 9.77 0 0 1 292.55 0Z"
      />
      <Text x={left + markWidth + 7} y={y} size={15} weight={600} fill={color.headings}>{node.title}</Text>
    </g>
  );
}

function Figure({ w, h, children }: { w: number; h: number; children: ReactNode }) {
  return (
    <svg viewBox={`0 0 ${w} ${h}`} role="img" aria-label={label} style={{ display: 'block', width: '100%', height: 'auto' }}>
      <title>{label}</title>
      {children}
    </svg>
  );
}

/** The three nodes stacked in a column `w` wide, each leg's request on the left of its arrows and
 * the answer on the right. 336px tall. */
function Stack({ w }: { w: number }) {
  const boxHeight = 64;
  const gap = 72;
  const ys = [0.5, 0.5 + boxHeight + gap, 0.5 + 2 * (boxHeight + gap)];
  const middle = w / 2;
  return (
    <g>
      {nodes.map((node, i) => (
        <g key={node.title}>
          <Box x={0.5} y={ys[i]} w={w - 1} h={boxHeight} accent={node.accent} />
          <NodeTitle x={middle} y={ys[i] + 28} node={node} />
          <Text x={middle} y={ys[i] + 48} anchor="middle" size={12.5}>{node.note}</Text>
        </g>
      ))}
      {legs.map((leg, i) => {
        const from = ys[i] + boxHeight + 10;
        const to = ys[i + 1] - 10;
        const baseline = (from + to) / 2 + 4;
        return (
          <g key={leg.request}>
            <Text x={middle - 20} y={baseline} anchor="end" code={leg.code}>{leg.request}</Text>
            <Arrow x1={middle - 8} y1={from} x2={middle - 8} y2={to} />
            <Arrow x1={middle + 8} y1={to} x2={middle + 8} y2={from} />
            <Text x={middle + 20} y={baseline}>{leg.answer}</Text>
          </g>
        );
      })}
    </g>
  );
}

const stackHeight = 3 * 64 + 2 * 72;
const cardHeight = 272;

/** The statement and the types TypeScript gets for it, wrapped to a card 328px wide. */
function Card({ x, y }: { x: number; y: number }) {
  const w = 328;
  const pad = 16;
  const code = { size: 12.5, fill: color.headings, code: true };
  return (
    <g transform={`translate(${x + 0.5} ${y})`}>
      <Box x={0} y={0} w={w - 1} h={cardHeight} />
      <Text x={pad} y={22} code>get_user.sql</Text>
      <Rule x={0} y={34} w={w - 1} />
      <Text x={pad} y={60} {...code} fill={color.text}>select id, name, email</Text>
      <Text x={pad} y={80} {...code} fill={color.text}>from users where id = :id</Text>
      <Rule x={0} y={98} w={w - 1} dashed />
      <Text x={pad} y={122}>parameters</Text>
      <Text x={pad} y={142} {...code}>{'{ id: number }'}</Text>
      <Text x={pad} y={170}>result</Text>
      <Text x={pad} y={190} {...code}>{'{ id: number;'}</Text>
      <Text x={pad} y={208} {...code}>{'  name: string;'}</Text>
      <Text x={pad} y={226} {...code}>{'  email: string | null } | null'}</Text>
      <Text x={pad} y={252}>{rowCount}</Text>
    </g>
  );
}

/**
 * The figure at the text column's width (640px, the 40rem column, at a 1440px window), drawn at
 * that size so its text is the size it was drawn at: the nodes stacked on the left and the
 * statement with its types beside them, on the right, so the figure meets both edges of the column.
 */
function Wide() {
  const w = 640;
  return (
    <Figure w={w} h={stackHeight + 1}>
      <Stack w={w - 328 - 16} />
      <Card x={w - 328} y={(stackHeight - cardHeight) / 2} />
    </Figure>
  );
}

/**
 * The same figure for a phone's column (328px on a 360px screen): the stack, and the statement
 * and its types under it.
 */
function Narrow() {
  const w = 328;
  const cardY = stackHeight + 24;
  return (
    <Figure w={w} h={cardY + cardHeight + 1}>
      <Stack w={w} />
      <Card x={0} y={cardY} />
    </Figure>
  );
}

/**
 * The diagram under How it works: a `.sql` import through TypeScript, Reylo and the database, and
 * the type that comes back. A container query picks the form: the wide one from a 560px column
 * (drawn for the page's 640px, where its text is the size it was drawn at, and no smaller than
 * 10.5px at 560px), and the stacked one below that, kept to 400px so that it does not grow larger
 * than the page's text on a small tablet.
 */
export function HowItWorksDiagram() {
  return (
    <figure className="@container my-8">
      <div className="hidden @min-[560px]:block">
        <Wide />
      </div>
      <div className="max-w-[400px] @min-[560px]:hidden">
        <Narrow />
      </div>
    </figure>
  );
}
