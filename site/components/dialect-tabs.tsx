'use client';

import { Tabs, TabsContent, TabsList, TabsTrigger } from 'fumadocs-ui/components/tabs';
import type { ReactNode } from 'react';

/*
 * One choice of dialect for the whole page. Fumadocs' code tabs marked `tab-group="dialect"`
 * become `<CodeBlockTabs groupId="dialect" persist>`, and every tabs root with a `groupId` joins
 * one listener set per id inside fumadocs-ui/dist/components/ui/tabs.js and saves the choice under
 * that id in sessionStorage and localStorage. These components are built on the same primitives
 * (the non-simple `Tabs`, whose values keep their case, unlike `<Tabs items>`, which lowercases
 * them), with the same `groupId` and `persist`, so a choice made in any of them, a code-tab strip
 * included, switches all of them and holds across reloads.
 */

export const DIALECTS = ['Postgres', 'MySQL', 'SQLite'] as const;
export type DialectName = (typeof DIALECTS)[number];

/** The storage key and listener group the code tabs use for `tab-group="dialect"`. */
const GROUP = 'dialect';

/**
 * `<DialectTabs>` shows a strip Postgres | MySQL | SQLite above the panel of the chosen dialect.
 * `<DialectTabs follow>` shows no strip, only the chosen dialect's panel: it follows the choice
 * made elsewhere on the page, as the TypeScript under an example's SQL does.
 */
export function DialectTabs({ follow = false, children }: { follow?: boolean; children: ReactNode }) {
  return (
    <Tabs
      groupId={GROUP}
      persist
      defaultValue={DIALECTS[0]}
      className={follow ? 'reylo-dialect-follow' : 'reylo-dialect-tabs'}
    >
      {!follow && (
        <TabsList>
          {DIALECTS.map((dialect) => (
            <TabsTrigger key={dialect} value={dialect}>
              {dialect}
            </TabsTrigger>
          ))}
        </TabsList>
      )}
      {children}
    </Tabs>
  );
}

/** One dialect's panel inside `<DialectTabs>`. */
export function Dialect({ value, children }: { value: DialectName; children: ReactNode }) {
  return (
    <TabsContent value={value} className="reylo-dialect-panel">
      {children}
    </TabsContent>
  );
}
