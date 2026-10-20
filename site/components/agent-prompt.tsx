'use client';

import { useRef, useState, type ReactNode } from 'react';

/**
 * The instructions a coding agent follows to set Reylo up in a project, at the top of Setup, as
 * the maintainer asked on 2026-10-20: "some kind of complete, copyable instruction block for
 * agents that's visually obvious". A card with one Copy button, which copies the whole prompt (the
 * code block given as children, as plain text), and the prompt below it, folded to its first lines
 * until it is opened.
 */
export function AgentPrompt({ children }: { children: ReactNode }) {
  const body = useRef<HTMLDivElement>(null);
  const [open, setOpen] = useState(false);
  const [copied, setCopied] = useState(false);

  async function copy() {
    // The code block's lines, as Shiki writes them: one `.line` element each.
    const lines = [...(body.current?.querySelectorAll('pre .line') ?? [])].map((line) => line.textContent ?? '');
    await navigator.clipboard.writeText(lines.join('\n').trimEnd() + '\n');
    setCopied(true);
    setTimeout(() => setCopied(false), 1600);
  }

  return (
    <section className="reylo-agent not-prose" aria-label="Setup instructions for a coding agent">
      <div className="reylo-agent-head">
        <div>
          <p className="reylo-agent-title">Set up Reylo with an agent</p>
          <p className="reylo-agent-note">Paste these instructions into Claude Code, Codex, Cursor or any coding agent.</p>
        </div>
        <button type="button" className="reylo-agent-copy" onClick={copy}>
          {copied ? 'Copied' : 'Copy instructions'}
        </button>
      </div>
      <div ref={body} className="reylo-agent-body" data-open={open || undefined}>
        {children}
      </div>
      <button type="button" className="reylo-agent-toggle" onClick={() => setOpen(!open)} aria-expanded={open}>
        {open ? 'Show less' : 'Show the instructions'}
      </button>
    </section>
  );
}
