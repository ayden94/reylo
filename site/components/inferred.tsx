import type { ReactNode } from 'react';
import { DIALECTS } from '@/lib/dialects';
import { DialectSwitch } from './dialect-tabs';

/*
 * The Inference section's cards: a statement with the types Reylo infers for it, in one card of
 * labelled rows, Schema (where the example needs one), Query, Params and Returns, every row
 * visible at once, scannable without a click on a tab. The page writes `<Inferred>` around a schema block
 * and a statement; lib/remark-mapper.ts fills it with a panel per dialect, the rows read off the
 * statement's captured module, which follow the page's dialect choice, and the switch here sets it.
 */

/** The card. `layout` is `side` (the parameters and the result beside the query) or `rows` (under it); `bare` drops the dialect switch. */
export function Inferred({ dialects, layout = 'side', bare, children }: { dialects?: string; layout?: 'side' | 'rows'; bare?: boolean; children: ReactNode }) {
  const shown = dialects ? DIALECTS.filter((d) => dialects.split(',').includes(d)) : DIALECTS;
  return (
    <div className="reylo-inferred" data-layout={layout}>
      {/* On a database's own page, `bare`: no switch, and no label naming the one database. */}
      {!bare && (
        <div className="reylo-inferred-switch">
          <DialectSwitch dialects={shown.length < DIALECTS.length ? shown.join(',') : undefined} />
        </div>
      )}
      {children}
    </div>
  );
}

/** One labelled row of the card: its label, then its code. */
export function InferredRow({ label, children }: { label: string; children: ReactNode }) {
  return (
    <div className="reylo-inferred-row" data-row={label.toLowerCase()}>
      <div className="reylo-inferred-label">{label}</div>
      <div className="reylo-inferred-body">{children}</div>
    </div>
  );
}

/** The Params and Returns rows together: a column beside the query, or a band under it. */
export function InferredTypes({ children }: { children: ReactNode }) {
  return <div className="reylo-inferred-types">{children}</div>;
}
