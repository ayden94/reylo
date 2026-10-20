import type { MetadataRoute } from 'next';
import { siteUrl } from '@/lib/layout.shared';
import { source } from '@/lib/source';

// The site's pages (one, at /), dated by the build, which runs on every push that changes the site.
export default function sitemap(): MetadataRoute.Sitemap {
  const lastModified = new Date();
  return source.getPages().map((page) => ({ url: new URL(page.url, siteUrl).href, lastModified, changeFrequency: 'weekly', priority: 1 }));
}
