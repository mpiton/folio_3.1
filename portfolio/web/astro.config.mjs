// @ts-check
import { defineConfig } from 'astro/config';
import tailwind from '@astrojs/tailwind';
import mdx from '@astrojs/mdx';
import sitemap from '@astrojs/sitemap';
import AstroPWA from '@vite-pwa/astro';
import pwaConfig from './src/pwa';

// https://astro.build/config
export default defineConfig({
  site: 'https://mathieu-piton.com',
  output: 'static', // Mode SSG
  build: {
    // Optimisations de build
    inlineStylesheets: 'auto',
    assets: 'assets'
  },
  integrations: [
    tailwind(),
    mdx(),
    sitemap({
      i18n: {
        defaultLocale: 'fr',
        locales: {
          fr: 'fr-FR',
          en: 'en-US'
        }
      }
    }),
    AstroPWA(pwaConfig)
  ],
  i18n: {
    defaultLocale: 'fr',
    locales: ['fr', 'en'],
    routing: {
      prefixDefaultLocale: false,
      strategy: 'pathname'
    }
  },
  // Configuration du prefetch natif d'Astro
  prefetch: {
    prefetchAll: true,
    defaultStrategy: 'hover'
  },
  vite: {
    build: {
      // Optimisations Vite
      cssCodeSplit: true,
      rollupOptions: {
        output: {
          manualChunks: {
            'vendor': ['three']
          }
        }
      }
    },
    // Configuration du cache
    optimizeDeps: {
      include: ['three'],
    },
    ssr: {
      noExternal: ['three']
    }
  }
});
