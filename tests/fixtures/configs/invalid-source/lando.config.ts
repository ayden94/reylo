// Deliberately broken: source.kind is invalid.
import { pg } from 'lando/pg';

export default {
  source: { kind: 'mongo', url: 'mongodb://localhost' },
  adapter: pg(),
};
