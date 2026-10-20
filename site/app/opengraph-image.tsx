import { socialCard } from '@/lib/social-card';

// The link preview (lib/social-card.tsx), which Next.js links from every page as og:image.
export { alt, size } from '@/lib/social-card';
export const contentType = 'image/png';

export default function Image() {
  return socialCard();
}
