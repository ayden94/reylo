import defaultMdxComponents from 'fumadocs-ui/mdx';
import { Accordion, Accordions } from 'fumadocs-ui/components/accordion';
import { Tab, Tabs } from 'fumadocs-ui/components/tabs';
import * as Twoslash from 'fumadocs-twoslash/ui';
import { Heading } from './heading';
import { Dialect, DialectTabs } from './dialect-tabs';
import { CodeTabs } from './code-tabs';
import { AgentPrompt } from './agent-prompt';
import { Details } from './details';
import { HowItWorksDiagram } from './how-it-works-diagram';
import type { MDXComponents } from 'mdx/types';
import type { ComponentProps } from 'react';

export function getMDXComponents(components?: MDXComponents) {
  return {
    ...defaultMdxComponents,
    // Section headings carry a link icon in the left margin (components/heading.tsx).
    h2: (props) => <Heading as="h2" {...props} />,
    h3: (props) => <Heading as="h3" {...props} />,
    h4: (props) => <Heading as="h4" {...props} />,
    Accordion,
    Accordions,
    // A secondary detail, opened on demand and drawn quietly (components/details.tsx).
    Details,
    // The setup instructions for a coding agent at the top of Setup (components/agent-prompt.tsx).
    AgentPrompt,
    // The figure under How it works (components/how-it-works-diagram.tsx).
    HowItWorksDiagram,
    Tab,
    Tabs,
    // One dialect choice for the page: <DialectTabs> with a strip, <DialectTabs follow> without,
    // each holding <Dialect value="Postgres|MySQL|SQLite"> panels, in step with the code tabs
    // marked tab-group="dialect" (components/dialect-tabs.tsx).
    DialectTabs,
    Dialect,
    // Code-tab groups put a file name the tabs share at the end of the strip (components/code-tabs.tsx).
    CodeBlockTabs: CodeTabs,
    // The hover popups of `ts twoslash` blocks.
    ...Twoslash,
    // Fumadocs waits 300ms before it opens a popup and as long before it closes one. 60ms opens it
    // as soon as the pointer settles on a name, and still leaves time to move into the popup.
    Popup: (props: ComponentProps<typeof Twoslash.Popup>) => <Twoslash.Popup {...props} delay={60} />,
    ...components,
  } satisfies MDXComponents;
}

export const useMDXComponents = getMDXComponents;

declare global {
  type MDXProvidedComponents = ReturnType<typeof getMDXComponents>;
}
