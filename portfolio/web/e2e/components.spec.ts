import { test, expect } from '@playwright/test';

test.describe('Components page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/components');
    await page.waitForLoadState('networkidle');
  });

  test('should display all required sections', async ({ page }) => {
    // Vérifier que le titre de la page est correct
    const title = page.getByRole('heading', { name: 'Documentation des Composants', level: 1 });
    await expect(title).toBeVisible();

    // Vérifier que les sections sont présentes
    const sections = [
      'Boutons',
      'Cartes',
      'Champs de formulaire',
      'Modales',
      'Notifications',
      'Loaders',
      'Pagination'
    ];

    for (const section of sections) {
      const sectionTitle = page.getByRole('heading', { name: section, level: 2 });
      await expect(sectionTitle).toBeVisible();
    }
  });

  test.describe('Button tests', () => {
    test('should display different button styles', async ({ page }) => {
      const buttonVariants = ['Primary', 'Secondary', 'Outline', 'Ghost'];
      for (const variant of buttonVariants) {
        const button = page.getByRole('button', { name: variant });
        await expect(button).toBeVisible();
      }
    });

    test('should handle button states', async ({ page }) => {
      // Test du bouton désactivé
      const disabledButton = page.getByRole('button', { name: 'Désactivé' });
      await expect(disabledButton).toBeVisible();
      await expect(disabledButton).toBeDisabled();

      // Test des tailles de boutons
      const buttonSizes = [
        { name: 'Small', class: 'sm' },
        { name: 'Medium', class: 'md' },
        { name: 'Large', class: 'lg' }
      ];
      for (const size of buttonSizes) {
        const button = page.locator(`.button--${size.class}`).first();
        await expect(button).toBeVisible();
      }
    });
  });

  test.describe('Card tests', () => {
    test('should display different card styles', async ({ page }) => {
      // Vérifier les cartes dans la section des boutons
      const buttonCards = page.locator('section', { hasText: 'Boutons' }).locator('.card');
      await expect(buttonCards).toHaveCount(3);

      // Vérifier que chaque carte a un titre
      const firstCard = buttonCards.first();
      await expect(firstCard.locator('h3')).toBeVisible();
    });
  });

  test.describe('Toast tests', () => {
    test.beforeEach(async ({ page }) => {
      // Attendre que la page soit complètement stable
      await page.waitForLoadState('networkidle');
      await page.waitForTimeout(2000);
    });

    test('should show and hide toast', async ({ page }) => {
      const showToastBtn = page.locator('[data-testid="showSuccessToast"]');
      await showToastBtn.click();

      // Attendre que le toast soit créé et visible avec un timeout plus long
      const toast = page.locator('.toast--success.toast--cloned').last();
      await expect(toast).toBeVisible({ timeout: 30000 });
      await expect(toast).toHaveClass(/toast--visible/, { timeout: 30000 });

      // Attendre que le toast disparaisse avec un timeout plus long
      await expect(toast).not.toBeVisible({ timeout: 35000 });
    });

    test('should show different toast types', async ({ page }) => {
      const toastTypes = ['Success', 'Error', 'Warning', 'Info'];

      for (const type of toastTypes) {
        const button = page.locator(`[data-testid="show${type}Toast"]`);
        await button.click();

        // Attendre que le toast soit créé et visible avec un timeout plus long
        await page.waitForSelector(`.toast--${type.toLowerCase()}.toast--cloned.toast--visible`, {
          state: 'visible',
          timeout: 30000
        });

        const toast = page.locator(`.toast--${type.toLowerCase()}.toast--cloned.toast--visible`).last();
        await expect(toast).toBeVisible({ timeout: 30000 });

        // Attendre que le toast disparaisse avec un timeout plus long
        await expect(toast).not.toBeVisible({ timeout: 35000 });

        // Attendre plus longtemps entre chaque toast
        await page.waitForTimeout(3000);
      }
    });

    test('should show multiple toasts simultaneously', async ({ page }) => {
      const toastTypes = ['Success', 'Error', 'Warning', 'Info'];

      // Attendre que la page soit complètement chargée
      await page.waitForLoadState('domcontentloaded');
      await page.waitForLoadState('networkidle');

      // Cliquer sur tous les boutons avec un délai entre chaque
      for (const type of toastTypes) {
        const button = page.getByTestId(`show${type}Toast`);
        await button.click();

        // Attendre que le toast soit créé et devienne visible
        await page.waitForSelector(`.toast--${type.toLowerCase()}.toast--cloned`, {
          state: 'attached',
          timeout: 15000
        });

        // Attendre que la classe toast--visible soit ajoutée
        await page.waitForFunction(
          (type) => {
            const toast = document.querySelector(`.toast--${type.toLowerCase()}.toast--cloned`);
            return toast && toast.classList.contains('toast--visible');
          },
          type.toLowerCase(),
          { timeout: 15000 }
        );
      }

      // Vérifier que tous les toasts sont visibles
      for (const type of toastTypes) {
        const toast = page.locator(`.toast--${type.toLowerCase()}.toast--cloned`).first();
        await expect(toast).toBeVisible({ timeout: 15000 });
        await expect(toast).toHaveClass(new RegExp(`toast--${type.toLowerCase()}.*toast--visible`), { timeout: 15000 });
      }
    });
  });

  test.describe('Modal tests', () => {
    test('should open and close modal', async ({ page }) => {
      const openModalBtn = page.getByRole('button', { name: 'Small Modal' });
      await openModalBtn.click();

      // Attendre que la modale soit visible
      const modal = page.locator('#smallModal');
      await expect(modal).toBeVisible({ timeout: 10000 });
      await expect(modal).toHaveAttribute('open', '');

      const closeModalBtn = modal.locator('[data-close-modal]');
      await closeModalBtn.click();

      // Attendre que la modale soit fermée
      await expect(modal).not.toBeVisible({ timeout: 10000 });
      await expect(modal).not.toHaveAttribute('open', '');
    });

    test('should close modal with overlay click', async ({ page }) => {
      const openModalBtn = page.getByTestId('openSmallModal');
      await openModalBtn.click();

      const modal = page.locator('#smallModal');
      await expect(modal).toHaveAttribute('open', '');

      // Attendre que la modale soit complètement ouverte
      await page.waitForTimeout(1000);

      // Cliquer en dehors de la modale (dans le coin supérieur gauche)
      await page.mouse.click(10, 10);

      // Attendre que la modale soit fermée
      await expect(modal).not.toHaveAttribute('open', '', { timeout: 10000 });
    });
  });
});
