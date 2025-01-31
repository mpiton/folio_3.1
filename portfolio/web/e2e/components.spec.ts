import { test, expect } from '@playwright/test';

test.describe('Components page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/components');
    await Promise.all([
      page.waitForLoadState('networkidle', { timeout: 60_000 }),
      page.waitForLoadState('domcontentloaded', { timeout: 60_000 })
    ]);
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
      const buttonTypes = ['Primary', 'Secondary', 'Outline', 'Ghost'];
      for (const type of buttonTypes) {
        const button = page.getByRole('button', { name: new RegExp(type, 'i') });
        await expect(button).toBeVisible({ timeout: 15_000 });
        const classes = await button.getAttribute('class');
        expect(classes).toContain(`button--${type.toLowerCase()}`);
      }
    });

    test('should handle button states', async ({ page }) => {
      const disabledButton = page.getByRole('button', { name: 'Désactivé' });
      await expect(disabledButton).toBeVisible();
      await expect(disabledButton).toBeDisabled();

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
      const cardTypes = ['default', 'elevated', 'bordered'];
      for (const type of cardTypes) {
        const card = page.locator(`[data-testid="card-${type}"]`);
        await expect(card).toBeVisible({ timeout: 15_000 });
        await expect(card).toHaveClass(new RegExp(`card--${type}`));
      }
    });
  });

  test.describe('Toast tests', () => {
    test.beforeEach(async ({ page }) => {
      await page.goto('/components');
      await Promise.all([
        page.waitForLoadState('networkidle', { timeout: 60_000 }),
        page.waitForLoadState('domcontentloaded', { timeout: 60_000 })
      ]);
      await page.waitForSelector('[data-testid="showSuccessToast"]', { state: 'visible', timeout: 15000 });
    });

    test('should show and hide toast', async ({ page }) => {
      const showToastBtn = page.getByTestId('showSuccessToast');
      await expect(showToastBtn).toBeVisible();
      await showToastBtn.click();

      const toast = page.locator('.toast--success.toast--cloned');
      await expect(toast).toBeAttached({ timeout: 5000 });
      await expect(toast).toHaveClass(/toast--visible/, { timeout: 5000 });

      // Vérifier que le toast disparaît après un certain temps
      await expect(toast).not.toBeVisible({ timeout: 10_000 });
    });

    test('should show different toast types', async ({ page }) => {
      const toastTypes = ['Success', 'Error', 'Warning', 'Info'];

      for (const type of toastTypes) {
        const button = page.getByTestId(`show${type}Toast`);
        await expect(button).toBeVisible();
        await button.click();

        const toast = page.locator(`.toast--${type.toLowerCase()}.toast--cloned`);
        await expect(toast).toBeAttached({ timeout: 5000 });
        await expect(toast).toHaveClass(/toast--visible/, { timeout: 5000 });
        await expect(toast).toHaveClass(new RegExp(`toast--${type.toLowerCase()}`));

        // Attendre que le toast disparaisse avant de passer au suivant
        await expect(toast).not.toBeVisible({ timeout: 10_000 });
        await page.waitForTimeout(1000);
      }
    });

    test('should show multiple toasts simultaneously', async ({ page }) => {
      const toastTypes = ['Success', 'Error', 'Warning', 'Info'];
      
      // Cliquer sur tous les boutons rapidement
      for (const type of toastTypes) {
        await page.getByTestId(`show${type}Toast`).click();
      }

      // Attendre que tous les toasts soient visibles
      for (const type of toastTypes) {
        const toast = page.locator(`.toast--${type.toLowerCase()}.toast--cloned`);
        await expect(toast).toBeAttached({ timeout: 15_000 });
        await expect(toast).toHaveClass(/toast--visible/, { timeout: 15_000 });
      }
    });
  });

  test.describe('Modal tests', () => {
    test('should open and close modal', async ({ page }) => {
      // Ouvrir la modale
      const openModalBtn = page.getByTestId('openSmallModal');
      await expect(openModalBtn).toBeVisible();
      await openModalBtn.click();

      // Attendre que la modale soit visible
      const modal = page.locator('#smallModal');
      await expect(modal).toBeVisible({ timeout: 5000 });

      // Vérifier le contenu de la modale
      const modalTitle = modal.getByRole('heading');
      await expect(modalTitle).toBeVisible();

      // Fermer la modale
      const closeButton = modal.getByRole('button', { name: /fermer/i });
      await expect(closeButton).toBeVisible();
      await closeButton.click();

      // Vérifier que la modale est fermée
      await expect(modal).not.toBeVisible({ timeout: 5000 });
    });

    test('should close modal with overlay click', async ({ page }) => {
      // Ouvrir la modale
      const openModalBtn = page.getByTestId('openSmallModal');
      await expect(openModalBtn).toBeVisible();
      await openModalBtn.click();

      // Attendre que la modale soit visible
      const modal = page.locator('#smallModal');
      await expect(modal).toBeVisible({ timeout: 5000 });

      // Cliquer à l'extérieur de la modale (en haut à gauche)
      await page.mouse.click(10, 10);

      // Vérifier que la modale est fermée
      await expect(modal).not.toBeVisible({ timeout: 5000 });
    });
  });
});
