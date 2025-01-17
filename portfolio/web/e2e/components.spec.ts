import { test, expect } from '@playwright/test';

test.describe('Components page', () => {
  test.beforeEach(async ({ page }) => {
    // Aller à la page des composants
    await page.goto('/components');

    // Attendre que la page soit complètement chargée
    await page.waitForLoadState('domcontentloaded');
    await page.waitForLoadState('networkidle');

    // Attendre que les scripts soient exécutés
    await page.waitForFunction(() => {
      return document.readyState === 'complete' &&
             !!document.querySelector('[data-testid]');
    });
  });

  test.describe('Modal tests', () => {
    test('should open and close small modal', async ({ page }) => {
      const openButton = page.getByTestId('openSmallModal');
      const smallModal = page.locator('#smallModal');

      await expect(openButton).toBeVisible({ timeout: 10000 });
      await openButton.click();

      await expect(smallModal).toBeVisible({ timeout: 10000 });
      await expect(smallModal.locator('h2')).toHaveText('Small Modal');

      const closeButton = smallModal.locator('[data-close-modal]');
      await closeButton.click();

      await expect(smallModal).not.toBeVisible();
    });

    test('should open and close medium modal', async ({ page }) => {
      const mediumModal = page.locator('#mediumModal');
      await expect(mediumModal).not.toBeVisible();

      const openButton = page.getByTestId('openMediumModal');
      await expect(openButton).toBeVisible({ timeout: 10000 });
      await openButton.click();

      await expect(mediumModal).toBeVisible({ timeout: 5000 });
      await expect(mediumModal.locator('h2')).toHaveText('Medium Modal');

      const closeButton = mediumModal.locator('[data-close-modal]');
      await expect(closeButton).toBeVisible();
      await closeButton.click();
      await expect(mediumModal).not.toBeVisible();
    });

    test('should open and close large modal', async ({ page }) => {
      const largeModal = page.locator('#largeModal');
      await expect(largeModal).not.toBeVisible();

      const openButton = page.getByTestId('openLargeModal');
      await expect(openButton).toBeVisible({ timeout: 10000 });
      await openButton.click();

      await expect(largeModal).toBeVisible({ timeout: 5000 });
      await expect(largeModal.locator('h2')).toHaveText('Large Modal');

      const closeButton = largeModal.locator('[data-close-modal]');
      await expect(closeButton).toBeVisible();
      await closeButton.click();
      await expect(largeModal).not.toBeVisible();
    });
  });

  test.describe('Toast tests', () => {
    test('should show and auto-dismiss success toast', async ({ page }) => {
      const showButton = page.getByTestId('showSuccessToast');
      await expect(showButton).toBeVisible({ timeout: 10000 });
      await showButton.click();

      const successToast = page.locator('.toast--success.toast--cloned.toast--visible');
      await expect(successToast).toBeVisible({ timeout: 5000 });
      await expect(successToast.locator('.toast-title')).toHaveText('Succès!');

      // Attendre que le toast disparaisse
      await expect(successToast).not.toBeVisible({ timeout: 10000 });
    });

    test('should show and manually close error toast', async ({ page }) => {
      const showButton = page.getByTestId('showErrorToast');
      await expect(showButton).toBeVisible({ timeout: 10000 });
      await showButton.click();

      const errorToast = page.locator('.toast--error.toast--cloned.toast--visible');
      await expect(errorToast).toBeVisible({ timeout: 5000 });

      const closeButton = errorToast.locator('[data-close-toast]');
      await expect(closeButton).toBeVisible();
      await closeButton.click();
      await expect(errorToast).not.toBeVisible({ timeout: 5000 });
    });

    test('should pause toast timer on hover', async ({ page }) => {
      const showButton = page.getByTestId('showWarningToast');
      await expect(showButton).toBeVisible({ timeout: 10000 });
      await showButton.click();

      const warningToast = page.locator('.toast--warning.toast--cloned.toast--visible');
      await expect(warningToast).toBeVisible({ timeout: 5000 });

      // Hover sur le toast
      await warningToast.hover();

      // Attendre 3 secondes
      await page.waitForTimeout(3000);

      // Le toast devrait toujours être visible
      await expect(warningToast).toBeVisible();

      // Retirer le hover
      await page.mouse.move(0, 0);

      // Le toast devrait disparaître après le délai restant
      await expect(warningToast).not.toBeVisible({ timeout: 7000 });
    });

    test('should show multiple toasts simultaneously', async ({ page }) => {
      const buttons = [
        'showSuccessToast',
        'showErrorToast',
        'showWarningToast'
      ];

      // Attendre que tous les boutons soient visibles
      for (const buttonId of buttons) {
        await expect(page.getByTestId(buttonId)).toBeVisible({ timeout: 10000 });
      }

      // Afficher les toasts
      for (const buttonId of buttons) {
        await page.getByTestId(buttonId).click();
        await page.waitForTimeout(200); // Délai plus long entre les toasts
      }

      // Vérifier que tous les toasts sont visibles
      const toastSelectors = [
        '.toast--success.toast--cloned.toast--visible',
        '.toast--error.toast--cloned.toast--visible',
        '.toast--warning.toast--cloned.toast--visible'
      ];

      for (const selector of toastSelectors) {
        await expect(page.locator(selector)).toBeVisible({ timeout: 5000 });
      }

      // Vérifier qu'ils sont empilés correctement
      const toasts = await page.locator('.toast--cloned.toast--visible').all();
      expect(toasts.length).toBe(3);
    });
  });
});
