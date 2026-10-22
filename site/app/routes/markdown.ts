import { markdownOf } from '@/lib/markdown';

// One page as Markdown, at /index.md for the main page and /<slug>.md for each other page of
// content/docs (lib/markdown.ts), written to build/client at build time. vercel.json answers a
// request for the page's own address that asks for Markdown (`Accept: text/markdown`, as Claude
// Code, Cursor and OpenCode send) with this file, as docs-kit's README describes.
export async function loader({ request }: { request: Request }) {
  // The address is /<name>.md, or /<name>.md.data when the build asks for the route's data.
  const name = new URL(request.url).pathname.replace(/^\/|\.md(?:\.data)?$/g, '');
  return new Response(await markdownOf(name === 'index' ? '' : name), {
    headers: { 'Content-Type': 'text/markdown; charset=utf-8' },
  });
}
