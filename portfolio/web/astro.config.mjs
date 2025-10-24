// @ts-check
import { defineConfig } from 'astro/config';
import mdx from '@astrojs/mdx';
import sitemap from '@astrojs/sitemap';
import AstroPWA from '@vite-pwa/astro';
import tailwindcss from '@tailwindcss/vite';

// https://astro.build/config
export default defineConfig({
  site: 'https://mathieu-piton.com',
  output: 'static', // Mode SSG
  build: {
    // Optimisations de build
    inlineStylesheets: 'auto',
    assets: 'assets',
    format: 'file'
  },
  vite: {
    // @ts-expect-error - Incompatibilité de types connue entre @tailwindcss/vite et Astro
    plugins: [tailwindcss()],
    build: {
      cssMinify: true,
      minify: 'terser',
      terserOptions: {
        compress: {
          // Keep important logs visible in prod
          pure_funcs: ['console.debug', 'console.info', 'console.trace'],
          drop_debugger: true
        }
      },
      rollupOptions: {
        output: {
          manualChunks: {
            'vendor': ['@fortawesome/fontawesome-free']
          }
        }
      }
    },
    ssr: {
      noExternal: ['@fortawesome/fontawesome-free']
    }
  },
  integrations: [
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
    AstroPWA({
      registerType: 'autoUpdate',
      manifest: {
        name: 'Mathieu Piton - Portfolio',
        short_name: 'MP Portfolio',
        description: 'Portfolio de Mathieu Piton, développeur Full Stack',
        theme_color: '#578E7E',
        background_color: '#F5ECD5',
        display: 'standalone',
        icons: [
          {
            src: '/icon-192x192.png',
            sizes: '192x192',
            type: 'image/png'
          },
          {
            src: '/icon-512x512.png',
            sizes: '512x512',
            type: 'image/png'
          },
          {
            src: '/icon-512x512.png',
            sizes: '512x512',
            type: 'image/png',
            purpose: 'maskable'
          }
        ],
        start_url: '/',
        scope: '/'
      },
      workbox: {
        navigateFallback: '/index.html',
        globPatterns: ['**/*.{css,js,html,svg,png,jpg,jpeg,gif,webp,woff,woff2,ttf,eot,ico}']
      },
      devOptions: {
        // Enable with `PWA_DEV=true npm run dev` when you need to test SW in dev
        enabled: process.env.PWA_DEV === 'true'
      }
    })
  ]
});
