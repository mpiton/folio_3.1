// @ts-check
import { defineConfig } from 'astro/config';
import tailwind from '@astrojs/tailwind';
import mdx from '@astrojs/mdx';
import sitemap from '@astrojs/sitemap';

// https://astro.build/config
export default defineConfig({
  site: 'https://mathieu-piton.com',
  integrations: [
    tailwind(),
    mdx(),
    sitemap(),
  ],
  i18n: {
    defaultLocale: 'fr',
    locales: ['fr', 'en'],
  },
  // Configuration du prefetch natif d'Astro
  prefetch: {
    prefetchAll: true,
    defaultStrategy: 'hover'
  }
});
