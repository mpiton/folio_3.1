import { defineConfig, devices } from '@playwright/test';

/**
 * Read environment variables from file.
 * https://github.com/motdotla/dotenv
 */
// import dotenv from 'dotenv';
// import path from 'path';
// dotenv.config({ path: path.resolve(__dirname, '.env') });

/**
 * See https://playwright.dev/docs/test-configuration.
 */
export default defineConfig({
  testDir: './e2e',
  /* Run tests in files in parallel */
  fullyParallel: false, // Désactiver le parallélisme pour plus de stabilité
  /* Fail the build on CI if you accidentally left test.only in the source code. */
  forbidOnly: !!process.env.CI,
  /* Retry failed tests */
  retries: process.env.CI ? 2 : 1,
  /* Reduce parallel workers for better stability */
  workers: 1, // Un seul worker pour éviter les problèmes de ressources
  /* Reporter to use. See https://playwright.dev/docs/test-reporters */
  reporter: [
    ['html'],
    ['list'] // Reporter en ligne de commande pour un meilleur feedback
  ],
  /* Shared settings for all the projects below. See https://playwright.dev/docs/api/class-testoptions. */
  use: {
    /* Base URL to use in actions like `await page.goto('/')`. */
    baseURL: 'http://localhost:4321',

    /* Collect trace when retrying the failed test. See https://playwright.dev/docs/trace-viewer */
    trace: 'retain-on-failure',

    /* Augmenter les timeouts */
    actionTimeout: 45000,
    navigationTimeout: 60000,

    /* Capture screenshot on failure */
    screenshot: 'only-on-failure',

    /* Record video for failed tests */
    video: 'retain-on-failure',
  },

  /* Global timeout par test */
  timeout: 120000,

  /* Configure projects for major browsers */
  projects: [
    {
      name: 'chromium',
      use: { 
        ...devices['Desktop Chrome'],
        viewport: { width: 1280, height: 720 },
        launchOptions: {
          args: ['--no-sandbox', '--disable-setuid-sandbox']
        }
      },
    },

    {
      name: 'firefox',
      use: { ...devices['Desktop Firefox'] },
    },

    {
      name: 'webkit',
      use: { ...devices['Desktop Safari'] },
    },
  ],

  /* Run your local dev server before starting the tests */
  webServer: [
    {
      // Démarrer d'abord le backend
      command: 'cd ../api && cargo run --bin portfolio-api',
      url: 'http://localhost:8080/health',
      reuseExistingServer: !process.env.CI,
      timeout: 180000, // 3 minutes pour le démarrage du serveur
      stdout: 'pipe',
      stderr: 'pipe'
    },
    {
      // Ensuite démarrer le frontend
      // On attend 5 secondes pour s'assurer que le backend est bien démarré
      command: 'timeout /t 5 && npm run dev',
      url: 'http://localhost:4321',
      reuseExistingServer: !process.env.CI,
      timeout: 180000, // 3 minutes pour le démarrage du serveur
      stdout: 'pipe',
      stderr: 'pipe'
    }
  ],

  /* Configuration globale */
  expect: {
    timeout: 30000, // Timeout par défaut pour les assertions
    toHaveScreenshot: {
      maxDiffPixels: 100, // Plus tolérant pour les différences de pixels
    },
  },
});
