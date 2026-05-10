import type { LandoConfig } from 'lando';
import { pg } from 'lando/pg';

export default {
  source: { kind: 'introspection', url: 'postgres://localhost:5432/example' },
  adapter: pg(),
  introspectAt: 'build',
  validate: { input: 'off' },
} satisfies LandoConfig;
