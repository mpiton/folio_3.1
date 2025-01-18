import { test, expect } from '@playwright/test';

test.describe('Contact Form', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/contact');
    await page.waitForLoadState('networkidle');
  });

  test('should show validation errors for empty required fields', async ({ page }) => {
    // Click submit without filling any fields
    await page.getByTestId('contact-submit').click();

    // Check for error messages with increased timeout
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

    // Fill form with valid data
    await page.getByTestId('contact-name').fill('Test User');
    await page.getByTestId('contact-email').fill('test@example.com');
    await page.getByTestId('contact-subject').fill('Test Subject');
    await page.getByTestId('contact-message').fill('Test message content');

    // Submit form and wait for response
    await Promise.all([
        page.waitForResponse('**/api/contact'),
        page.getByTestId('contact-submit').click()
    ]);

    // Attendre que le toast cloné soit créé et visible
    await page.waitForSelector('.toast--success.toast--cloned', {
        state: 'attached',
        timeout: 10000
    });
    const successToast = page.locator('.toast--success.toast--cloned').first();
    await expect(successToast).toBeVisible({ timeout: 10000 });

    // Verify form was reset
    await expect(page.getByTestId('contact-name')).toHaveValue('');
    await expect(page.getByTestId('contact-email')).toHaveValue('');
    await expect(page.getByTestId('contact-subject')).toHaveValue('');
    await expect(page.getByTestId('contact-message')).toHaveValue('');
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

    // Submit form and wait for response
    const responsePromise = page.waitForResponse('**/api/contact');
    await page.getByTestId('contact-submit').click();
    const response = await responsePromise;

    // Verify response status
    expect(response.status()).toBe(500);

    // Log response body for debugging
    console.log('API Response:', await response.json());

    // Attendre que le toast soit créé
    await page.waitForSelector('.toast--error.toast--cloned', {
        state: 'attached',
        timeout: 15000
    });

    // Attendre que l'animation soit complètement terminée
    await page.waitForFunction(
        () => {
            const toast = document.querySelector('.toast--error.toast--cloned');
            if (!toast) return false;
            const style = window.getComputedStyle(toast);
            const transform = style.transform || style.webkitTransform;
            const opacity = parseFloat(style.opacity);

            // Log pour le débogage
            console.log('Animation check:', {
                transform,
                opacity,
                hasVisibleClass: toast.classList.contains('toast--visible')
            });

            return opacity === 1 && transform === 'matrix(1, 0, 0, 1, 0, 0)';
        },
        {
            timeout: 15000,
            polling: 100  // Vérifier toutes les 100ms
        }
    );

    // Vérifier que le toast est visible
    const toast = page.locator('.toast--error.toast--cloned').first();
    await expect(toast).toBeVisible();

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
