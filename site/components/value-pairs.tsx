import { Children, isValidElement, type ReactElement, type ReactNode } from 'react';
import './value-pairs.css';

/*
 * A rule list with value pairs, for the dialect pages' rule lists, labelled "SQL, JavaScript". Inside
 * `<Rules>`, each rule is a bold line naming the case, then its discussion, then `<Pairs>`: a list
 * of SQL values and what `.run()` returns for each, one per line as `SQL` → `JavaScript`, which
 * `<Pairs>` draws as a card of labelled rows. The page stays Markdown, so the copy of a section
 * and llms.txt keep every term, sentence and pair.
 *
 * <Rules>
 *
 * **Fractions of a second**
 *
 * The fraction is the server's, to the microsecond.
 *
 * <Pairs>
 *
 * - `'2026-10-20 10:00:00.5+00'::timestamptz` → `"2026-10-20T10:00:00.5Z"`
 *
 * </Pairs>
 *
 * </Rules>
 */

/** The list: each bold line in it is a case, set as a term (value-pairs.css). */
export function Rules({ children }: { children: ReactNode }) {
  return <div className="reylo-rules">{children}</div>;
}

/** The pairs of a rule: each list item's text is `SQL → JavaScript`. */
export function Pairs({ children }: { children: ReactNode }) {
  const pairs = items(children).map((item) => {
    const text = textOf(item);
    const at = text.indexOf('→');
    return at < 0 ? { sql: text.trim(), js: '' } : { sql: text.slice(0, at).trim(), js: text.slice(at + 1).trim() };
  });
  return (
    <div className="reylo-pairs not-prose">
      {pairs.map((pair, i) => (
        <div className="reylo-pair" key={i}>
          <div className="reylo-pair-label">SQL</div>
          <code className="reylo-pair-code">{sql(pair.sql)}</code>
          <div className="reylo-pair-label">JavaScript</div>
          <code className="reylo-pair-code">{js(pair.js)}</code>
        </div>
      ))}
    </div>
  );
}

/** The `<li>` elements under the list `<Pairs>` holds, whatever the MDX components wrap them in. */
function items(node: ReactNode): ReactElement[] {
  const found: ReactElement[] = [];
  const walk = (n: ReactNode) =>
    Children.forEach(n, (child) => {
      if (!isValidElement(child)) return;
      const props = child.props as { children?: ReactNode };
      if (child.type === 'li') found.push(child);
      else walk(props.children);
    });
  walk(node);
  return found;
}

function textOf(node: ReactNode): string {
  if (node == null || typeof node === 'boolean') return '';
  if (typeof node === 'string' || typeof node === 'number') return String(node);
  if (Array.isArray(node)) return node.map(textOf).join('');
  if (isValidElement(node)) return textOf((node.props as { children?: ReactNode }).children);
  return '';
}

/** SQL, coloured as the page's code blocks colour it: strings, a cast's type, functions, numbers. */
function sql(text: string): ReactNode[] {
  return tokens(text, /('(?:[^']|'')*')|::([a-z_][a-z0-9_ ]*?(?:\[\])?)(?=$|[^a-z0-9_])|\b([a-z_][a-z0-9_]*)(?=\()|(\b\d[\d.]*\b)/gi, (m) =>
    m[1] ? ['str', m[1]] : m[2] ? ['type', `::${m[2]}`, '::'] : m[3] ? ['fn', m[3]] : ['num', m[4]!],
  );
}

/** A JavaScript value: strings, numbers and bigints, and a thrown error. */
function js(text: string): ReactNode[] {
  if (text.startsWith('throws')) return [<span key="e" className="reylo-pair-error">{text}</span>];
  return tokens(text, /("(?:[^"\\]|\\.)*")|((?<![\w.])-?\d[\d.]*n?(?![\w.]))/g, (m) => (m[1] ? ['str', m[1]] : ['num', m[2]!]));
}

type Token = [kind: string, text: string, plain?: string];

function tokens(text: string, pattern: RegExp, kind: (m: RegExpExecArray) => Token): ReactNode[] {
  const out: ReactNode[] = [];
  let last = 0;
  for (let m = pattern.exec(text); m; m = pattern.exec(text)) {
    if (m.index > last) out.push(text.slice(last, m.index));
    const [k, t, plain] = kind(m);
    // A cast keeps its `::` in the text's colour and colours the type alone.
    if (plain) out.push(plain);
    out.push(
      <span key={m.index} className={`reylo-pair-${k}`}>
        {plain ? t.slice(plain.length) : t}
      </span>,
    );
    last = m.index + m[0].length;
  }
  if (last < text.length) out.push(text.slice(last));
  return out;
}
