import { CodeBlockTabs, CodeBlockTabsList } from 'fumadocs-ui/components/codeblock';
import { Children, Fragment, cloneElement, isValidElement, type ComponentProps, type ReactElement, type ReactNode } from 'react';

type Element = ReactElement<{ value?: string; title?: string; children?: ReactNode }>;

/** The elements among `children`, with fragments opened: MDX wraps a tab's block in one. */
const elements = (children: ReactNode): Element[] =>
  (Children.toArray(children).filter(isValidElement) as Element[]).flatMap((child) => (child.type === Fragment ? elements(child.props.children) : [child]));

/**
 * Fumadocs' code-tab group (`CodeBlockTabs`, which remark-code-tab writes for consecutive blocks
 * with a `tab`), with one change: when every tab's block carries the same `title`, as an example's
 * `title="get_user.sql" tab="Postgres"` blocks do, the file name moves to the right end of the tab
 * strip and leaves the blocks, so the group has one bar, the dialects on the left and the file on
 * the right, instead of the strip above a second bar with the name. Each block keeps its copy
 * button, which Fumadocs floats over the code of a block without a title. A group whose tabs carry
 * different titles, or none (the hero: `main.ts`, `get_user.sql`), is left as Fumadocs draws it.
 */
export function CodeTabs({ children, className, ...props }: ComponentProps<typeof CodeBlockTabs>) {
  const parts = elements(children);
  const tabs = parts.filter((part) => part.props.value !== undefined);
  const blocks = tabs.map((tab) => elements(tab.props.children));
  const titles = new Set(blocks.map((inner) => (inner.length === 1 ? inner[0].props.title : undefined)));
  const [title] = titles;
  const classes = ['reylo-code-tabs', className].filter(Boolean).join(' ');
  if (titles.size !== 1 || typeof title !== 'string' || title === '') {
    return <CodeBlockTabs {...props} className={classes}>{children}</CodeBlockTabs>;
  }
  return (
    <CodeBlockTabs {...props} className={classes} data-file={title}>
      {parts.map((part, i) => {
        if (part.props.value === undefined) {
          // The strip: Fumadocs' triggers, then the file name.
          return (
            <CodeBlockTabsList key={part.key ?? i}>
              {part.props.children}
              <span className="reylo-code-tabs-file" aria-label={`File: ${title}`}>{title}</span>
            </CodeBlockTabsList>
          );
        }
        return cloneElement(part, { key: part.key ?? i }, elements(part.props.children).map((block, j) => cloneElement(block, { key: block.key ?? j, title: undefined })));
      })}
    </CodeBlockTabs>
  );
}
