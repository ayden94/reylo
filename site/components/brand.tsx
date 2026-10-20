import { Wordmark } from '@/components/wordmark';

/** The wordmark, a link to the top of the page: docs-kit's chrome shows it at the top of the left column, or above the title below xl. */
export function Brand() {
  return (
    <a href="/" className="inline-block text-fd-headings">
      <Wordmark className="h-6 w-auto" />
    </a>
  );
}
