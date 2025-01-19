import { test, expect } from '@playwright/test';

test.describe('Contact Form', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/contact');
    await page.waitForLoadState('networkidle');
  });

  test('should show validation errors for empty required fields', async ({ page }) => {
    // Attendre que le formulaire soit chargé
    await page.waitForSelector('[data-testid="contact-submit"]', { state: 'visible', timeout: 10000 });

    // Click submit without filling any fields
    await page.getByTestId('contact-submit').click();

    // Check for error messages with increased timeout
    await page.waitForSelector('.error-message', { state: 'visible', timeout: 10000 });
    const errorMessages = await page.locator('.error-message').all();
    expect(errorMessages).toHaveLength(4); // name, email, subject, message
  });

  test('should show validation error for invalid email', async ({ page }) => {
    // Fill form with invalid email
    await page.getByTestId('contact-name').fill('Test User');
    await page.getByTestId('contact-email').fill('invalid-email');

    // Trigger blur event and wait for validation
    await page.getByTestId('contact-email').evaluate(e => e.blur());

    // Attendre que le champ soit marqué comme invalide
    await expect(page.getByTestId('contact-email')).toHaveClass(/error/, { timeout: 5000 });

    // Vérifier que le conteneur a la classe error
    const emailContainer = page.locator('.input-container', { has: page.getByTestId('contact-email') });
    await expect(emailContainer).toHaveClass(/error/, { timeout: 5000 });

    // Check for email error message with increased timeout
    const emailError = await page.locator('.error-message').filter({ hasText: /email.*valide/i });
    await expect(emailError).toBeVisible({ timeout: 15000 });
  });

  test('should submit form successfully and show success toast', async ({ page }) => {
    // Mock the API response
    await page.route('**/api/contact', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ message: 'Success' })
      });
    });

    // Attendre que le formulaire soit complètement chargé
    await page.waitForSelector('[data-testid="contact-submit"]', {
      state: 'visible',
      timeout: 30000
    });

    // Fill form with valid data
    await page.getByTestId('contact-name').fill('Test User');
    await page.getByTestId('contact-email').fill('test@example.com');
    await page.getByTestId('contact-subject').fill('Test Subject');
    await page.getByTestId('contact-message').fill('Test message content');

    // Attendre que tous les champs soient remplis et stables
    await page.waitForTimeout(1000);

    // Submit form and wait for response
    await Promise.all([
      page.waitForResponse('**/api/contact'),
      page.getByTestId('contact-submit').click()
    ]);

    // Attendre que le toast soit créé et visible avec un timeout plus long
    await page.waitForSelector('.toast--success.toast--cloned', {
      state: 'attached',
      timeout: 30000
    });

    // Attendre que la classe toast--visible soit ajoutée
    await page.waitForFunction(
      () => {
        const toast = document.querySelector('.toast--success.toast--cloned');
        return toast && toast.classList.contains('toast--visible');
      },
      { timeout: 30000 }
    );

    const successToast = page.locator('.toast--success.toast--cloned').first();
    await expect(successToast).toBeVisible({ timeout: 30000 });
    await expect(successToast).toHaveClass(/toast--success.*toast--visible/, { timeout: 30000 });

    // Verify form was reset with longer timeout
    await expect(page.getByTestId('contact-name')).toHaveValue('', { timeout: 15000 });
    await expect(page.getByTestId('contact-email')).toHaveValue('', { timeout: 15000 });
    await expect(page.getByTestId('contact-subject')).toHaveValue('', { timeout: 15000 });
    await expect(page.getByTestId('contact-message')).toHaveValue('', { timeout: 15000 });
  });

  test('should show error toast when submission fails', async ({ page }) => {
    // Mock the API response with an error
    await page.route('**/api/contact', async route => {
      await route.fulfill({
        status: 500,
        contentType: 'application/json',
        body: JSON.stringify({
          message: 'Une erreur est survenue'
        })
      });
    });

    // Fill form with valid data
    await page.getByTestId('contact-name').fill('Test User');
    await page.getByTestId('contact-email').fill('test@example.com');
    await page.getByTestId('contact-subject').fill('Test Subject');
    await page.getByTestId('contact-message').fill('Test message content');

    // Attendre que tous les champs soient remplis
    await page.waitForTimeout(1000);

    // Submit form and wait for response
    const [response] = await Promise.all([
      page.waitForResponse('**/api/contact'),
      page.getByTestId('contact-submit').click()
    ]);

    // Verify response status
    expect(response.status()).toBe(500);

    // Attendre que le toast soit créé et visible
    await page.waitForSelector('.toast--error.toast--cloned.toast--visible', {
      state: 'visible',
      timeout: 15000
    });

    // Vérifier que le toast est visible
    const toast = page.locator('.toast--error.toast--cloned.toast--visible').first();
    await expect(toast).toBeVisible({ timeout: 15000 });

    // Verify form was not reset
    await expect(page.getByTestId('contact-name')).toHaveValue('Test User');
    await expect(page.getByTestId('contact-email')).toHaveValue('test@example.com');
    await expect(page.getByTestId('contact-subject')).toHaveValue('Test Subject');
    await expect(page.getByTestId('contact-message')).toHaveValue('Test message content');
  });
});

test.describe('Contact Form Validation', () => {
    // Augmenter les retries spécifiquement pour ce groupe de tests
    test.describe.configure({ retries: 3 });

    test.beforeEach(async ({ page }) => {
        // Attendre que le serveur soit prêt avant de naviguer
        await test.step('Wait for server and navigate', async () => {
            let retries = 0;
            while (retries < 3) {
                try {
                    await page.goto('/contact', {
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

    test('should validate form submission with French text', async ({ page }) => {
        // Attendre que la page soit stable
        await page.waitForLoadState('networkidle');

        // Remplir le formulaire avec du texte français valide
        await page.getByTestId('contact-name').fill("Jean-Pierre d'Artagnan");
        await page.getByTestId('contact-email').fill("test@example.com");
        await page.getByTestId('contact-subject').fill("Test d'intégration");
        await page.getByTestId('contact-message').fill("Je teste l'envoi d'un message avec des caractères français : é, è, à, ç. C'est important !");

        // Attendre que les champs soient remplis
        await page.waitForTimeout(500);

        // Déclencher les événements blur et attendre la validation
        for (const field of ['contact-name', 'contact-email', 'contact-subject', 'contact-message']) {
            await page.getByTestId(field).evaluate(e => e.blur());
            await page.waitForTimeout(100);
        }

        // Vérifier qu'il n'y a pas d'erreurs de validation
        const errorMessages = await page.locator('.error-message').all();
        expect(errorMessages.length).toBe(0);

        // Soumettre le formulaire
        await page.getByTestId('contact-submit').click();
    });
});
