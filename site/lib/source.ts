import { defineDocs } from 'fumadocs-mdx/macro';
import { loader } from 'fumadocs-core/source';

// content/docs/index.mdx at / and content/docs/details.mdx at /details.
const docs = defineDocs({
  dir: 'content/docs',
});

export const source = loader({
  baseUrl: '/',
  source: docs.toFumadocsSource(),
});
