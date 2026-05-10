// Deliberately broken: missing the default export.
import type { LandoConfig } from 'lando';

export const config: LandoConfig = {
  source: { kind: 'prisma', path: './schema.prisma' },
  adapter: { kind: 'pg', capabilities: { checkConstraints: true, expressionIndexes: true, lateral: true, skipLocked: true } },
};
