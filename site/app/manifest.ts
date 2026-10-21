import type { MetadataRoute } from 'next';

// For a browser that saves the site to a home screen: the name and the tile (public/icon-*.png,
// written by brand/mark.ts), on the page's own background.
export default function manifest(): MetadataRoute.Manifest {
  return {
    name: 'Reylo',
    short_name: 'Reylo',
    description: 'Import .sql files into TypeScript with full type inference.',
    start_url: '/',
    display: 'browser',
    background_color: '#0e0e0e',
    theme_color: '#0e0e0e',
    icons: [
      { src: '/icon-192.png', sizes: '192x192', type: 'image/png' },
      { src: '/icon-512.png', sizes: '512x512', type: 'image/png' },
    ],
  };
}
