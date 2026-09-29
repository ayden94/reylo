// Placeholder lexing for the runtime loader: finds `:name`, `$name`, `$1`, `?` and `?NNN`
// outside strings and comments and returns the binding order. A name may be dotted
// (`:auth.user_id`); each segment starts like a name, so `arr[1:2]` is a slice. This is the
// provisional JavaScript version of the engine's lexer; the Rust engine owns the
// authoritative one (dollar quoting, E'' strings, nested comments, array slices).
export type Placeholder = { name: string; start: number; end: number };

export function lexPlaceholders(sql: string): { params: string[]; placeholders: Placeholder[] } {
  const placeholders: Placeholder[] = [];
  const params: string[] = [];
  let positional = 0;
  let i = 0;
  const push = (name: string, start: number, end: number) => {
    placeholders.push({ name, start, end });
    if (!params.includes(name)) params.push(name);
  };
  while (i < sql.length) {
    const ch = sql[i]!;
    const next = sql[i + 1];
    if (ch === "'" ) { i = skipQuoted(sql, i, "'"); continue; }
    if (ch === '"') { i = skipQuoted(sql, i, '"'); continue; }
    if (ch === "-" && next === "-") { i = sql.indexOf("\n", i); if (i === -1) break; continue; }
    if (ch === "/" && next === "*") { const end = sql.indexOf("*/", i + 2); i = end === -1 ? sql.length : end + 2; continue; }
    if (ch === ":" && next === ":") { i += 2; continue; }
    if ((ch === ":" || ch === "$" || ch === "@") && next !== undefined && /[A-Za-z_]/.test(next)) {
      const m = /^[A-Za-z_][A-Za-z0-9_]*(?:\.[A-Za-z_][A-Za-z0-9_]*)*/.exec(sql.slice(i + 1))!;
      push(m[0], i, i + 1 + m[0].length); i += 1 + m[0].length; continue;
    }
    if (ch === "$" && next !== undefined && /[0-9]/.test(next)) {
      const m = /^[0-9]+/.exec(sql.slice(i + 1))!;
      push(`$${m[0]}`, i, i + 1 + m[0].length); i += 1 + m[0].length; continue;
    }
    if (ch === "?" && next !== undefined && /[0-9]/.test(next)) {
      // SQLite's `?NNN` names the same position a `$NNN` does.
      const m = /^[0-9]+/.exec(sql.slice(i + 1))!;
      push(`$${m[0]}`, i, i + 1 + m[0].length); i += 1 + m[0].length; continue;
    }
    if (ch === "?") { positional += 1; push(`?${positional}`, i, i + 1); i += 1; continue; }
    i += 1;
  }
  return { params, placeholders };
}

function skipQuoted(sql: string, start: number, quote: string): number {
  let i = start + 1;
  while (i < sql.length) {
    if (sql[i] === quote) { if (sql[i + 1] === quote) { i += 2; continue; } return i + 1; }
    i += 1;
  }
  return i;
}
