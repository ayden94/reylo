import { Variant, VariantTabs } from 'docs-kit/components';
import type { ReactNode } from 'react';
import { DIALECTS, type DialectName } from '@/lib/dialects';

/*
 * One choice of dialect for the whole page: docs-kit's variant tabs with the group `dialect`, the
 * same group as Fumadocs' code tabs marked `tab-group="dialect"`, so a choice made in any of them,
 * a code-tab strip included, switches all of them and holds across reloads.
 */

/**
 * `<DialectTabs>` shows a strip Postgres | MySQL | SQLite above the panel of the chosen dialect.
 * `<DialectTabs follow>` shows no strip, only the chosen dialect's panel: it follows the choice
 * made elsewhere on the page, as the TypeScript under an example's SQL does (lib/remark-mapper.ts
 * writes those).
 */
export function DialectTabs({ follow = false, children }: { follow?: boolean; children: ReactNode }) {
  return (
    <VariantTabs group="dialect" values={DIALECTS} follow={follow}>
      {children}
    </VariantTabs>
  );
}

/** One dialect's panel inside `<DialectTabs>`. */
export function Dialect({ value, children }: { value: DialectName; children: ReactNode }) {
  return <Variant value={value}>{children}</Variant>;
}
