// Configuration PWA
export default {
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
        type: 'image/png',
      },
      {
        src: '/icon-512x512.png',
        sizes: '512x512',
        type: 'image/png',
      },
      {
        src: '/icon-512x512.png',
        sizes: '512x512',
        type: 'image/png',
        purpose: 'maskable',
      },
    ],
    start_url: '/',
    scope: '/',
  },
  workbox: {
    navigateFallback: process.env.NODE_ENV === 'production' ? '/404' : null,
    globPatterns:
      process.env.NODE_ENV === 'production'
        ? ['**/*.{css,js,html,svg,png,jpg,jpeg,gif,webp,woff,woff2,ttf,eot,ico}']
        : [],
    runtimeCaching: [
      {
        urlPattern: /^https:\/\/api\.mathieu-piton\.com\/.*$/,
        handler: 'NetworkFirst',
        options: {
          cacheName: 'api-cache',
          networkTimeoutSeconds: 5,
          expiration: {
            maxEntries: 50,
            maxAgeSeconds: 60 * 60 * 24, // 24 heures
          },
          cacheableResponse: {
            statuses: [0, 200],
          },
        },
      },
      {
        urlPattern: /^https:\/\/fonts\.googleapis\.com\/.*/i,
        handler: 'CacheFirst',
        options: {
          cacheName: 'google-fonts-cache',
          expiration: {
            maxEntries: 10,
            maxAgeSeconds: 60 * 60 * 24 * 365, // 1 an
          },
        },
      },
    ],
  },
  devOptions: {
    enabled: true,
    type: 'module',
    navigateFallback: null,
  },
};
