import { defineDocs } from 'fumadocs-mdx/macro';
import { loader } from 'fumadocs-core/source';

// One page: content/docs/index.mdx, served at /.
const docs = defineDocs({
  dir: 'content/docs',
});

export const source = loader({
  baseUrl: '/',
  source: docs.toFumadocsSource(),
});
