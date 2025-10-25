import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./src/tests/setup.ts'],
    include: ['src/**/*.{test,spec}.{js,ts}'],
    exclude: ['node_modules/', 'dist/', 'e2e/', 'tests/'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html', 'clover'],
      reportsDirectory: './coverage',
      include: ['src/**/*.{ts,tsx,js,jsx}'],
      exclude: [
        'node_modules/',
        'tests/',
        'e2e/',
        'dist/',
        'src/tests/',
        'src/**/*.{test,spec}.{ts,tsx,js,jsx}',
        'src/env.d.ts',
      ],
    },
  },
});
