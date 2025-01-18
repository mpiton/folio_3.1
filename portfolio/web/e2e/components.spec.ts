import { test, expect } from '@playwright/test';

test.describe('Components page', () => {
  test.beforeEach(async ({ page }) => {
    // Attendre que le serveur soit prêt avant de naviguer
    await test.step('Wait for server and navigate', async () => {
      let retries = 0;
      while (retries < 3) {
        try {
          await page.goto('/components', {
            timeout: 30000,
            waitUntil: 'networkidle'
          });
          break;
        } catch (e) {
          retries++;
          if (retries === 3) throw e;
          // Attendre un peu avant de réessayer
          await page.waitForTimeout(1000);
        }
      }
    });

    // S'assurer que la page est complètement chargée
    await page.waitForLoadState('domcontentloaded');
    await page.waitForLoadState('networkidle');
  });

  test.describe('Modal tests', () => {
    test('should open and close small modal', async ({ page }) => {
      // Attendre que la page soit stable
      await page.waitForLoadState('networkidle');

      const openButton = page.getByTestId('openSmallModal');
      const smallModal = page.locator('#smallModal');

      // Attendre que le bouton soit stable et cliquable
      await expect(openButton).toBeVisible({ timeout: 15000 });
      await expect(openButton).toBeEnabled();

      // Cliquer et attendre l'animation
      await openButton.click();
      await page.waitForTimeout(500);

      // Vérifier que le modal est visible et a le bon contenu
      await expect(smallModal).toBeVisible({ timeout: 15000 });
      await expect(smallModal.locator('h2')).toHaveText('Small Modal');

      // Attendre que le bouton de fermeture soit stable
      const closeButton = smallModal.locator('[data-close-modal]');
      await expect(closeButton).toBeVisible({ timeout: 5000 });
      await expect(closeButton).toBeEnabled();

      // Fermer et attendre l'animation
      await closeButton.click();
      await page.waitForTimeout(500);

      // Vérifier que le modal est bien fermé
      await expect(smallModal).not.toBeVisible({ timeout: 5000 });
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
      // Attendre que la page soit stable
      await page.waitForLoadState('networkidle');

      const largeModal = page.locator('#largeModal');
      await expect(largeModal).not.toBeVisible();

      // Attendre que le bouton soit stable et cliquable
      const openButton = page.getByTestId('openLargeModal');
      await expect(openButton).toBeVisible({ timeout: 15000 });
      await expect(openButton).toBeEnabled();

      // Cliquer et attendre l'animation
      await openButton.click();
      await page.waitForTimeout(500);

      // Vérifier que le modal est visible et a le bon contenu
      await expect(largeModal).toBeVisible({ timeout: 15000 });
      await expect(largeModal.locator('h2')).toHaveText('Large Modal');

      // Attendre que le bouton de fermeture soit stable
      const closeButton = largeModal.locator('[data-close-modal]');
      await expect(closeButton).toBeVisible({ timeout: 5000 });
      await expect(closeButton).toBeEnabled();

      // Fermer et attendre l'animation
      await closeButton.click();
      await page.waitForTimeout(500);

      // Vérifier que le modal est bien fermé
      await expect(largeModal).not.toBeVisible({ timeout: 5000 });
    });
  });

  test.describe('Toast tests', () => {
    test('should show and auto-dismiss success toast', async ({ page }) => {
      // Attendre que la page soit stable
      await page.waitForLoadState('networkidle');

      // Attendre que le bouton soit stable et cliquable
      const showButton = page.getByTestId('showSuccessToast');
      await expect(showButton).toBeVisible({ timeout: 15000 });
      await expect(showButton).toBeEnabled();

      // Cliquer et attendre l'animation
      await showButton.click();
      await page.waitForTimeout(500);

      // Attendre que le toast soit créé et visible
      const successToast = page.locator('.toast--success.toast--cloned');
      await expect(successToast).toBeVisible({ timeout: 15000 });
      await expect(successToast).toHaveClass(/toast--visible/, { timeout: 5000 });
      await expect(successToast.locator('.toast-title')).toHaveText('Succès!');

      // Attendre que le toast disparaisse
      await expect(successToast).not.toBeVisible({ timeout: 15000 });
    });

    test('should show and manually close error toast', async ({ page }) => {
      // Attendre que la page soit stable
      await page.waitForLoadState('networkidle');

      // Attendre que le bouton soit stable et cliquable
      const showButton = page.getByTestId('showErrorToast');
      await expect(showButton).toBeVisible({ timeout: 15000 });
      await expect(showButton).toBeEnabled();

      // Cliquer et attendre l'animation
      await showButton.click();
      await page.waitForTimeout(500);

      // Attendre que le toast soit créé et visible
      const errorToast = page.locator('.toast--error.toast--cloned');
      await expect(errorToast).toBeVisible({ timeout: 15000 });
      await expect(errorToast).toHaveClass(/toast--visible/, { timeout: 5000 });

      // Attendre que le bouton de fermeture soit stable
      const closeButton = errorToast.locator('[data-close-toast]');
      await expect(closeButton).toBeVisible({ timeout: 5000 });
      await expect(closeButton).toBeEnabled();

      // Fermer et attendre l'animation
      await closeButton.click();
      await page.waitForTimeout(500);

      // Vérifier que le toast est bien fermé
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
      // Attendre que la page soit stable
      await page.waitForLoadState('networkidle');

      const buttons = [
        'showSuccessToast',
        'showErrorToast',
        'showWarningToast'
      ];

      // Attendre que tous les boutons soient visibles et cliquables
      for (const buttonId of buttons) {
        const button = page.getByTestId(buttonId);
        await expect(button).toBeVisible({ timeout: 15000 });
        await expect(button).toBeEnabled();
      }

      // Afficher les toasts avec un délai entre chaque
      for (const buttonId of buttons) {
        await page.getByTestId(buttonId).click();
        await page.waitForTimeout(500); // Délai plus long entre les toasts
      }

      // Attendre que tous les toasts soient créés et visibles
      const toastTypes = ['success', 'error', 'warning'];
      for (const type of toastTypes) {
        const toast = page.locator(`.toast--${type}.toast--cloned`);
        await expect(toast).toBeVisible({ timeout: 15000 });
        await expect(toast).toHaveClass(/toast--visible/, { timeout: 5000 });
      }

      // Vérifier qu'ils sont empilés correctement
      const toasts = await page.locator('.toast--cloned.toast--visible').all();
      expect(toasts.length).toBe(3);

      // Attendre que les toasts soient stables avant de terminer le test
      await page.waitForTimeout(500);
    });
  });
});

// Configurer les retries au niveau du test
test.describe.configure({ retries: 2 });
