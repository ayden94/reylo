import type { LandoConfig } from 'lando';
import { pg } from 'lando/pg';

export default {
  source: { kind: 'prisma', path: './prisma/schema.prisma' },
  adapter: pg(),
  polymorphism: {
    Account: {
      kind: 'inheritance',
      discriminator: 'kind',
      strategy: 'single-table',
      variants: {
        user: { include: ['displayName'] },
        admin: { include: ['permissions'] },
      },
    },
  },
} satisfies LandoConfig;
