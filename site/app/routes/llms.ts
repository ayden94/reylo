import { markdownOf } from '@/lib/markdown';

// Every page as Markdown, for a coding agent or a language model (https://llmstxt.org): the main
// page, then each dialect's reference, in the order a reader takes them, each as its own Markdown
// file holds it (lib/markdown.ts). Written to build/client/llms.txt at build time.
const PAGES = ['', 'postgres', 'mysql', 'sqlite'];

export async function loader() {
  const text = (await Promise.all(PAGES.map(markdownOf))).join('\n');
  return new Response(text, { headers: { 'Content-Type': 'text/markdown; charset=utf-8' } });
}
