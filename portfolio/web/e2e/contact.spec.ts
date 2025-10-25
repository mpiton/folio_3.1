import { test, expect } from '@playwright/test';
import './test-types';

declare global {
  interface Window {
    ENV: {
      MODE: string;
    };
    ToastManager: unknown;
  }
}

test.describe('Contact Form', () => {
  test.beforeEach(async ({ page }) => {
    // Définir la variable d'environnement pour le mode test
    await page.addInitScript(() => {
      window.import = {
        meta: {
          env: {
            PUBLIC_API_URL: 'http://localhost:8080',
            MODE: 'test',
          },
        },
      };
    });

    // Mock pour l'API de contact
    await page.route('**/api/contact', async route => {
      const request = route.request();
      if (request.method() === 'POST') {
        const body = JSON.parse(request.postData() || '{}');

        // Attendre un peu pour simuler le délai réseau
        await new Promise(resolve => setTimeout(resolve, 1000));

        if (body.email === 'error@example.com') {
          await route.fulfill({
            status: 500,
            contentType: 'application/json',
            body: JSON.stringify({
              status: 'error',
              message: 'Internal Server Error',
            }),
          });
          return;
        }

        await route.fulfill({
          status: 200,
          contentType: 'application/json',
          body: JSON.stringify({
            status: 'success',
            message: 'Message envoyé avec succès',
          }),
        });
      } else {
        await route.continue();
      }
    });

    await page.goto('/contact');

    // Attendre que la page soit chargée
    await Promise.all([
      page.waitForLoadState('networkidle', { timeout: 60000 }),
      page.waitForLoadState('domcontentloaded'),
    ]);
  });

  test('should submit form successfully', async ({ page }) => {
    // Remplir le formulaire
    await page.getByLabel('Nom').fill('Test User');
    await page.getByLabel('Email').fill('test@example.com');
    await page.getByLabel('Message').fill('Test message');

    // Soumettre le formulaire
    const submitButton = page.getByRole('button', { name: 'Envoyer' });
    await expect(submitButton).toBeEnabled();
    await submitButton.click();

    // Attendre et vérifier le message de succès
    const successMessage = page.locator('.toast--success.toast--cloned');
    await expect(successMessage).toBeVisible({ timeout: 15000 });
    await expect(successMessage).toHaveClass(/toast--visible/, { timeout: 15000 });
    await expect(successMessage).toContainText('Message envoyé avec succès');

    // Vérifier que le formulaire est réinitialisé
    await expect(page.getByLabel('Nom')).toHaveValue('', { timeout: 15000 });
    await expect(page.getByLabel('Email')).toHaveValue('', { timeout: 15000 });
    await expect(page.getByLabel('Message')).toHaveValue('', { timeout: 15000 });
  });

  test('should handle submission error', async ({ page }) => {
    // Remplir le formulaire avec l'email d'erreur
    await page.getByLabel('Nom').fill('Error User');
    await page.getByLabel('Email').fill('error@example.com');
    await page.getByLabel('Message').fill('Error test message');

    // Soumettre le formulaire
    const submitButton = page.getByRole('button', { name: 'Envoyer' });
    await expect(submitButton).toBeEnabled();
    await submitButton.click();

    // Attendre et vérifier le message d'erreur
    const errorMessage = page.locator('.toast--error.toast--cloned');
    await expect(errorMessage).toBeVisible({ timeout: 15000 });
    await expect(errorMessage).toHaveClass(/toast--visible/, { timeout: 15000 });
    await expect(errorMessage).toContainText('Internal Server Error');

    // Vérifier que le formulaire conserve les valeurs
    await expect(page.getByLabel('Nom')).toHaveValue('Error User', { timeout: 15000 });
    await expect(page.getByLabel('Email')).toHaveValue('error@example.com', { timeout: 15000 });
    await expect(page.getByLabel('Message')).toHaveValue('Error test message', { timeout: 15000 });
  });
});
