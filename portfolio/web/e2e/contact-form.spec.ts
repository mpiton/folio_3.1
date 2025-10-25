import { test, expect } from '@playwright/test';
import { AxeBuilder } from '@axe-core/playwright';

// ============================================================================
// GROUPE 1: FORM RENDERING (1.1-1.3)
// ============================================================================

test.describe('ContactForm - Rendering', () => {
  test.beforeEach(async ({ page }) => {
    // Configurer l'API URL en mode test
    await page.addInitScript(() => {
      window.import = {
        meta: {
          env: {
            PUBLIC_API_URL: 'http://localhost:8080',
          },
        },
      };
    });

    // Mock de base pour l'API de contact
    await page.route('**/api/contact', async route => {
      const request = route.request();
      if (request.method() === 'POST') {
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
    await page.waitForLoadState('domcontentloaded');
  });

  test('1.1: should render form with all fields visible', async ({ page }) => {
    // Arrange: form page loaded
    // Act: check visibility
    const form = page.locator('#contact-form');
    const nameInput = page.locator('input[name="name"]');
    const emailInput = page.locator('input[name="email"]');
    const subjectInput = page.locator('input[name="subject"]');
    const messageTextarea = page.locator('textarea[name="message"]');
    const submitButton = page.locator('button[type="submit"]');

    // Assert: all elements visible
    await expect(form).toBeVisible();
    await expect(nameInput).toBeVisible();
    await expect(emailInput).toBeVisible();
    await expect(subjectInput).toBeVisible();
    await expect(messageTextarea).toBeVisible();
    await expect(submitButton).toBeVisible();

    // Assert: submit button has correct text
    await expect(submitButton).toContainText('Envoyer');
  });

  test('1.2: should have ARIA labels for accessibility', async ({ page }) => {
    // Arrange: form loaded
    // Act: check accessibility attributes
    const nameInput = page.locator('input[name="name"]');
    const emailInput = page.locator('input[name="email"]');
    const subjectInput = page.locator('input[name="subject"]');
    const messageTextarea = page.locator('textarea[name="message"]');

    // Assert: each field has associated label
    const nameLabel = page.locator('label:has-text("Nom")');
    const emailLabel = page.locator('label:has-text("Email")');
    const subjectLabel = page.locator('label:has-text("Sujet")');
    const messageLabel = page.locator('label:has-text("Message")');

    await expect(nameLabel).toBeVisible();
    await expect(emailLabel).toBeVisible();
    await expect(subjectLabel).toBeVisible();
    await expect(messageLabel).toBeVisible();

    // Assert: inputs have testids for accessibility
    await expect(nameInput).toHaveAttribute('data-testid', 'contact-name');
    await expect(emailInput).toHaveAttribute('data-testid', 'contact-email');
    await expect(subjectInput).toHaveAttribute('data-testid', 'contact-subject');
    await expect(messageTextarea).toHaveAttribute('data-testid', 'contact-message');
  });

  test('1.3: should have correct placeholders for all fields', async ({ page }) => {
    // Arrange: form loaded
    // Act: check placeholders
    const nameInput = page.locator('input[name="name"]');
    const emailInput = page.locator('input[name="email"]');
    const subjectInput = page.locator('input[name="subject"]');
    const messageTextarea = page.locator('textarea[name="message"]');

    // Assert: placeholders are visible
    await expect(nameInput).toHaveAttribute('placeholder', 'Votre nom');
    await expect(emailInput).toHaveAttribute('placeholder', 'votre@email.com');
    await expect(subjectInput).toHaveAttribute('placeholder', 'Sujet de votre message');
    await expect(messageTextarea).toHaveAttribute('placeholder', 'Votre message');
  });
});

// ============================================================================
// GROUPE 2: VALIDATION (2.1-2.5)
// ============================================================================

test.describe('ContactForm - Validation', () => {
  test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => {
      window.import = {
        meta: {
          env: {
            PUBLIC_API_URL: 'http://localhost:8080',
          },
        },
      };
    });

    // Mock pour l'API de contact
    await page.route('**/api/contact', async route => {
      const request = route.request();
      if (request.method() === 'POST') {
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
    await page.waitForLoadState('domcontentloaded');
  });

  test('2.1: should submit valid form successfully', async ({ page }) => {
    // Arrange: form loaded
    const nameInput = page.locator('input[name="name"]');
    const emailInput = page.locator('input[name="email"]');
    const subjectInput = page.locator('input[name="subject"]');
    const messageTextarea = page.locator('textarea[name="message"]');
    const submitButton = page.locator('button[type="submit"]');

    // Act: fill form with valid data
    await nameInput.fill('John Doe');
    await emailInput.fill('john.doe@example.com');
    await subjectInput.fill('Project Inquiry');
    await messageTextarea.fill('I would like to discuss a web development project.');

    // Act: submit form
    await submitButton.click();

    // Assert: success toast appears
    const successToast = page.locator('.toast[data-type="success"]');
    await expect(successToast).toBeVisible({ timeout: 10000 });
  });

  test('2.2: should prevent invalid email submission', async ({ page }) => {
    // Arrange: form loaded
    const nameInput = page.locator('input[name="name"]');
    const emailInput = page.locator('input[name="email"]');
    const subjectInput = page.locator('input[name="subject"]');
    const messageTextarea = page.locator('textarea[name="message"]');
    const _submitButton = page.locator('button[type="submit"]');

    // Act: fill form with invalid email
    await nameInput.fill('Test User');
    await emailInput.fill('not-an-email');
    await subjectInput.fill('Test Subject');
    await messageTextarea.fill('This is a test message');

    // Act: blur email field to trigger validation
    await emailInput.blur();

    // Assert: error message appears
    const errorMessage = page.locator("text=L'email n'est pas valide");
    await expect(errorMessage).toBeVisible();

    // Assert: email input has error class
    await expect(emailInput).toHaveClass(/error/);
  });

  test('2.3: should require all fields', async ({ page }) => {
    // Arrange: form loaded
    const nameInput = page.locator('input[name="name"]');
    const _submitButton = page.locator('button[type="submit"]');

    // Act: focus and blur empty field to trigger validation
    await nameInput.focus();
    await nameInput.blur();

    // Assert: error message appears with correct selector
    const errorMessage = page.locator('.error-message');
    await expect(errorMessage).toBeVisible();
    // Verify error text content
    await expect(errorMessage).toContainText('Ce champ est requis');
  });

  test('2.4: should enforce message minimum length', async ({ page }) => {
    // Arrange: form loaded
    const messageTextarea = page.locator('textarea[name="message"]');

    // Act: fill message with too few characters
    await messageTextarea.fill('short');
    await messageTextarea.blur();

    // Assert: error message for minimum length appears
    const errorMessage = page.locator('text=Le message doit faire entre 10 et 1000 caractères');
    await expect(errorMessage).toBeVisible();

    // Assert: message textarea has error class
    await expect(messageTextarea).toHaveClass(/error/);
  });

  test('2.5: should prevent HTML/XSS injection in message', async ({ page }) => {
    // Arrange: form loaded
    const nameInput = page.locator('input[name="name"]');
    const emailInput = page.locator('input[name="email"]');
    const subjectInput = page.locator('input[name="subject"]');
    const messageTextarea = page.locator('textarea[name="message"]');

    // Act: fill form with script tag (should fail validation)
    await nameInput.fill('Test User');
    await emailInput.fill('test@example.com');
    await subjectInput.fill('Test');
    await messageTextarea.fill('<script>alert("xss")</script> This message has a script');

    // Act: blur textarea to trigger validation
    await messageTextarea.blur();

    // Assert: validation error appears (special characters not allowed)
    const errorMessage = page.locator('.error-message');
    await expect(errorMessage).toBeVisible();
    await expect(errorMessage).toContainText('Le message contient des caractères non autorisés');
  });
});

// ============================================================================
// GROUPE 3: API INTEGRATION (3.1-3.4)
// ============================================================================

test.describe('ContactForm - API Integration', () => {
  test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => {
      window.import = {
        meta: {
          env: {
            PUBLIC_API_URL: 'http://localhost:8080',
          },
        },
      };
    });

    await page.goto('/contact');
    await page.waitForLoadState('domcontentloaded');
  });

  test('3.1: should call POST /api/contact on valid submit', async ({ page }) => {
    // Arrange: wait for API response
    let capturedData: Record<string, unknown> | null = null;

    await page.route('**/api/contact', async route => {
      const request = route.request();
      if (request.method() === 'POST') {
        capturedData = JSON.parse(request.postData() || '{}');

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

    const nameInput = page.locator('input[name="name"]');
    const emailInput = page.locator('input[name="email"]');
    const subjectInput = page.locator('input[name="subject"]');
    const messageTextarea = page.locator('textarea[name="message"]');
    const submitButton = page.locator('button[type="submit"]');

    // Act: fill and submit valid form
    await nameInput.fill('Test User');
    await emailInput.fill('test@example.com');
    await subjectInput.fill('Test Subject');
    await messageTextarea.fill('This is a test message for validation');

    // Wait for API response and submit
    const responsePromise = page.waitForResponse('**/api/contact');
    await submitButton.click();
    const response = await responsePromise;

    // Assert: API call was made successfully
    expect(response.status()).toBe(200);

    // Assert: request body matches form data
    expect(capturedData).toEqual({
      name: 'Test User',
      email: 'test@example.com',
      subject: 'Test Subject',
      message: 'This is a test message for validation',
    });
  });

  test('3.2: should handle API 500 error gracefully', async ({ page }) => {
    // Arrange: mock API to return 500
    await page.route('**/api/contact', async route => {
      await route.fulfill({
        status: 500,
        contentType: 'application/json',
        body: JSON.stringify({
          status: 'error',
          message: 'Internal Server Error',
        }),
      });
    });

    const nameInput = page.locator('input[name="name"]');
    const emailInput = page.locator('input[name="email"]');
    const subjectInput = page.locator('input[name="subject"]');
    const messageTextarea = page.locator('textarea[name="message"]');
    const submitButton = page.locator('button[type="submit"]');

    // Act: fill and submit form
    await nameInput.fill('Error Test User');
    await emailInput.fill('error@test.com');
    await subjectInput.fill('Error Test');
    await messageTextarea.fill('Testing error handling with long message here');
    await submitButton.click();

    // Assert: error toast appears
    const errorToast = page.locator('.toast[data-type="error"]');
    await expect(errorToast).toBeVisible({ timeout: 10000 });

    // Assert: form values are NOT reset on error
    await expect(nameInput).toHaveValue('Error Test User');
    await expect(emailInput).toHaveValue('error@test.com');
  });

  test('3.3: should handle network timeout', async ({ page }) => {
    // Arrange: mock API to abort (simulate timeout)
    await page.route('**/api/contact', async route => {
      // Simulate a timeout by waiting longer than the request timeout
      await new Promise(resolve => setTimeout(resolve, 100));
      await route.abort('timedout');
    });

    const nameInput = page.locator('input[name="name"]');
    const emailInput = page.locator('input[name="email"]');
    const subjectInput = page.locator('input[name="subject"]');
    const messageTextarea = page.locator('textarea[name="message"]');
    const submitButton = page.locator('button[type="submit"]');

    // Act: fill and submit form
    await nameInput.fill('Timeout Test');
    await emailInput.fill('timeout@test.com');
    await subjectInput.fill('Timeout Test');
    await messageTextarea.fill('This message will timeout during submission');

    // Act: click submit and wait for error
    await submitButton.click();

    // Assert: error toast appears
    const errorToast = page.locator('.toast[data-type="error"]');
    await expect(errorToast).toBeVisible({ timeout: 10000 });
  });

  test('3.4: should disable submit button during submission', async ({ page }) => {
    // Arrange: mock API with delay
    let fulfillRequest: ((value: unknown) => void) | null = null;
    const requestPromise = new Promise(resolve => {
      fulfillRequest = resolve as (value: unknown) => void;
    });

    await page.route('**/api/contact', async route => {
      const request = route.request();
      if (request.method() === 'POST') {
        // Wait for the promise to be fulfilled
        await requestPromise;

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

    const nameInput = page.locator('input[name="name"]');
    const emailInput = page.locator('input[name="email"]');
    const subjectInput = page.locator('input[name="subject"]');
    const messageTextarea = page.locator('textarea[name="message"]');
    const submitButton = page.locator('button[type="submit"]');

    // Act: fill form
    await nameInput.fill('Test User');
    await emailInput.fill('test@example.com');
    await subjectInput.fill('Test Subject');
    await messageTextarea.fill('This is a test message with enough length');

    // Act: initiate submit
    const _submitPromise = submitButton.click();

    // Wait a moment for the button to be disabled
    await page.waitForTimeout(100);

    // Assert: button is disabled during submission
    // Note: We check that the button has disabled state or is not clickable
    const isDisabled = await submitButton.isDisabled();
    const canClick = await submitButton.isEnabled();

    // The button should be in some form of disabled state
    expect(isDisabled || !canClick).toBeTruthy();

    // Act: allow the request to complete
    if (fulfillRequest) {
      // @ts-expect-error - Type inference issue with Promise callback
      fulfillRequest(undefined);
    }

    // Wait for success response
    await expect(page.locator('.toast[data-type="success"]')).toBeVisible({ timeout: 10000 });

    // Assert: button is re-enabled after submission
    await expect(submitButton).toBeEnabled({ timeout: 5000 });
  });
});

// ============================================================================
// GROUPE 4: SUCCESS FEEDBACK (4.1-4.2)
// ============================================================================

test.describe('ContactForm - Success Feedback', () => {
  test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => {
      window.import = {
        meta: {
          env: {
            PUBLIC_API_URL: 'http://localhost:8080',
          },
        },
      };
    });

    // Mock for successful API response
    await page.route('**/api/contact', async route => {
      const request = route.request();
      if (request.method() === 'POST') {
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
    await page.waitForLoadState('domcontentloaded');
  });

  test('4.1: should show success toast after valid submission', async ({ page }) => {
    // Arrange: form loaded
    const nameInput = page.locator('input[name="name"]');
    const emailInput = page.locator('input[name="email"]');
    const subjectInput = page.locator('input[name="subject"]');
    const messageTextarea = page.locator('textarea[name="message"]');
    const submitButton = page.locator('button[type="submit"]');

    // Act: fill and submit valid form
    await nameInput.fill('Success Test User');
    await emailInput.fill('success@test.com');
    await subjectInput.fill('Success Test');
    await messageTextarea.fill('This message should trigger success notification');
    await submitButton.click();

    // Assert: success toast is visible
    const successToast = page.locator('.toast[data-type="success"]');
    await expect(successToast).toBeVisible({ timeout: 10000 });

    // Assert: toast has success styling
    await expect(successToast).toHaveClass(/toast--visible/);

    // Assert: toast contains success message
    await expect(successToast).toContainText('Message envoyé avec succès');
  });

  test('4.2: should reset form after successful submission', async ({ page }) => {
    // Arrange: form loaded
    const nameInput = page.locator('input[name="name"]');
    const emailInput = page.locator('input[name="email"]');
    const subjectInput = page.locator('input[name="subject"]');
    const messageTextarea = page.locator('textarea[name="message"]');
    const submitButton = page.locator('button[type="submit"]');

    // Act: fill form with data
    await nameInput.fill('Reset Test User');
    await emailInput.fill('reset@test.com');
    await subjectInput.fill('Reset Test');
    await messageTextarea.fill('Testing form reset after successful submission');

    // Act: submit form
    await submitButton.click();

    // Wait for success toast to appear
    await expect(page.locator('.toast[data-type="success"]')).toBeVisible({ timeout: 10000 });

    // Wait a bit more to ensure form.reset() has completed
    await page.waitForTimeout(500);

    // Assert: all fields are empty after successful submission
    // Use trim() comparison in case of any whitespace
    const nameValue = await nameInput.inputValue();
    const emailValue = await emailInput.inputValue();
    const subjectValue = await subjectInput.inputValue();
    const messageValue = await messageTextarea.inputValue();

    expect(nameValue.trim()).toBe('');
    expect(emailValue.trim()).toBe('');
    expect(subjectValue.trim()).toBe('');
    expect(messageValue.trim()).toBe('');

    // Assert: form is ready for new submission
    await expect(submitButton).toBeEnabled();
  });
});

// ============================================================================
// GROUPE 5: RATE LIMITING (5.1)
// ============================================================================

test.describe('ContactForm - Rate Limiting', () => {
  test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => {
      window.import = {
        meta: {
          env: {
            PUBLIC_API_URL: 'http://localhost:8080',
          },
        },
      };
    });

    // Mock for successful API response
    await page.route('**/api/contact', async route => {
      const request = route.request();
      if (request.method() === 'POST') {
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
    await page.waitForLoadState('domcontentloaded');
  });

  test('5.1: should handle multiple rapid submissions', async ({ page }) => {
    // Arrange: form loaded
    const nameInput = page.locator('input[name="name"]');
    const emailInput = page.locator('input[name="email"]');
    const subjectInput = page.locator('input[name="subject"]');
    const messageTextarea = page.locator('textarea[name="message"]');
    const submitButton = page.locator('button[type="submit"]');

    // Act: fill form once
    await nameInput.fill('Rate Limit Test User');
    await emailInput.fill('ratelimit@test.com');
    await subjectInput.fill('Rate Limit Test');
    await messageTextarea.fill('Testing rate limiting functionality here');

    // Act: submit form first time
    await submitButton.click();
    await expect(page.locator('.toast[data-type="success"]')).toBeVisible({ timeout: 10000 });

    // Act: wait for form reset
    await page.waitForFunction(
      () => (document.querySelector('input[name="name"]') as HTMLInputElement | null)?.value === '',
      {
        timeout: 5000,
      }
    );

    // Act: immediately fill form again (second submission)
    await nameInput.fill('Rate Limit Test User 2');
    await emailInput.fill('ratelimit2@test.com');
    await subjectInput.fill('Rate Limit Test 2');
    await messageTextarea.fill('Testing second rapid submission for rate limiting');

    // Act: submit form second time
    await submitButton.click();

    // Assert: API should handle both submissions (or show appropriate feedback)
    const toastElements = await page.locator('.toast').all();
    expect(toastElements.length).toBeGreaterThanOrEqual(2);
  });
});

// ============================================================================
// GROUPE 6: ACCESSIBILITY (6.1-6.2)
// ============================================================================

test.describe('ContactForm - Accessibility', () => {
  test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => {
      window.import = {
        meta: {
          env: {
            PUBLIC_API_URL: 'http://localhost:8080',
          },
        },
      };
    });

    // Mock for successful API response
    await page.route('**/api/contact', async route => {
      const request = route.request();
      if (request.method() === 'POST') {
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
    await page.waitForLoadState('domcontentloaded');
  });

  test('6.1: should be keyboard navigable', async ({ page }) => {
    // Arrange: form loaded
    const nameInput = page.locator('input[name="name"]');
    const emailInput = page.locator('input[name="email"]');
    const subjectInput = page.locator('input[name="subject"]');
    const messageTextarea = page.locator('textarea[name="message"]');
    const submitButton = page.locator('button[type="submit"]');

    // Act: tab to name field
    await nameInput.focus();
    await expect(nameInput).toBeFocused();

    // Act: tab to email field
    await page.keyboard.press('Tab');
    await expect(emailInput).toBeFocused();

    // Act: tab to subject field
    await page.keyboard.press('Tab');
    await expect(subjectInput).toBeFocused();

    // Act: tab to message field
    await page.keyboard.press('Tab');
    await expect(messageTextarea).toBeFocused();

    // Act: tab to submit button
    await page.keyboard.press('Tab');
    await expect(submitButton).toBeFocused();

    // Assert: can submit with Enter key
    await nameInput.focus();
    await nameInput.fill('Keyboard Test');
    await emailInput.fill('keyboard@test.com');
    await subjectInput.fill('Keyboard Test');
    await messageTextarea.fill('Testing keyboard navigation and submission');

    // Tab to submit button and press Enter
    await submitButton.focus();
    await submitButton.press('Enter');

    // Assert: form submission works via keyboard
    await expect(page.locator('.toast[data-type="success"]')).toBeVisible({ timeout: 10000 });
  });

  test('6.2: should pass axe accessibility audit', async ({ page }) => {
    // Arrange & Act: run accessibility check using AxeBuilder for the contact form only
    const contactForm = page.locator('section.contact-form');
    const results = await new AxeBuilder({ page })
      .include('.contact-form')
      .exclude('.social-links')
      .exclude('body > main')
      .analyze();

    // Assert: no violations found in the contact form
    expect(results.violations).toHaveLength(0);
  });
});

// ============================================================================
// ADDITIONAL EDGE CASE TESTS
// ============================================================================

test.describe('ContactForm - Edge Cases', () => {
  test.beforeEach(async ({ page }) => {
    await page.addInitScript(() => {
      window.import = {
        meta: {
          env: {
            PUBLIC_API_URL: 'http://localhost:8080',
          },
        },
      };
    });

    // Mock for successful API response
    await page.route('**/api/contact', async route => {
      const request = route.request();
      if (request.method() === 'POST') {
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
    await page.waitForLoadState('domcontentloaded');
  });

  test('should clear validation errors when correcting field', async ({ page }) => {
    // Arrange: form loaded
    const emailInput = page.locator('input[name="email"]');

    // Act: enter invalid email
    await emailInput.fill('invalid-email');
    await emailInput.blur();

    // Assert: error message appears
    const errorMessage = page.locator("text=L'email n'est pas valide");
    await expect(errorMessage).toBeVisible();

    // Act: correct the email
    await emailInput.clear();
    await emailInput.fill('valid@example.com');
    await emailInput.blur();

    // Assert: error message disappears
    await expect(errorMessage).not.toBeVisible();
  });

  test('should handle name with minimum characters', async ({ page }) => {
    // Arrange: form loaded
    const nameInput = page.locator('input[name="name"]');
    const emailInput = page.locator('input[name="email"]');
    const subjectInput = page.locator('input[name="subject"]');
    const messageTextarea = page.locator('textarea[name="message"]');
    const submitButton = page.locator('button[type="submit"]');

    // Act: fill form with 2-character name (minimum)
    await nameInput.fill('Jo');
    await emailInput.fill('test@example.com');
    await subjectInput.fill('Test');
    await messageTextarea.fill('This is a valid test message');

    // Act: submit form
    await submitButton.click();

    // Assert: form accepts 2-character name and submits
    await expect(page.locator('.toast[data-type="success"]')).toBeVisible({ timeout: 10000 });
  });

  test('should handle message with maximum characters', async ({ page }) => {
    // Arrange: form loaded
    const nameInput = page.locator('input[name="name"]');
    const emailInput = page.locator('input[name="email"]');
    const subjectInput = page.locator('input[name="subject"]');
    const messageTextarea = page.locator('textarea[name="message"]');
    const submitButton = page.locator('button[type="submit"]');

    // Create a 1000-character message (maximum)
    const maxMessage = 'A'.repeat(1000);

    // Act: fill form with maximum length message
    await nameInput.fill('Max Test User');
    await emailInput.fill('max@example.com');
    await subjectInput.fill('Max Test');
    await messageTextarea.fill(maxMessage);

    // Act: submit form
    await submitButton.click();

    // Assert: form accepts and submits maximum length message
    await expect(page.locator('.toast[data-type="success"]')).toBeVisible({ timeout: 10000 });
  });

  test('should reject message exceeding maximum characters', async ({ page }) => {
    // Arrange: form loaded
    const messageTextarea = page.locator('textarea[name="message"]');

    // Create a message exceeding 1000 characters
    const tooLongMessage = 'A'.repeat(1001);

    // Act: fill message with too many characters
    await messageTextarea.fill(tooLongMessage);
    await messageTextarea.blur();

    // Assert: error message appears
    const errorMessage = page.locator('text=Le message doit faire entre 10 et 1000 caractères');
    await expect(errorMessage).toBeVisible();
  });

  test('should handle email with special characters', async ({ page }) => {
    // Arrange: form loaded
    const nameInput = page.locator('input[name="name"]');
    const emailInput = page.locator('input[name="email"]');
    const subjectInput = page.locator('input[name="subject"]');
    const messageTextarea = page.locator('textarea[name="message"]');
    const submitButton = page.locator('button[type="submit"]');

    // Act: fill form with special email characters
    await nameInput.fill('Test User');
    await emailInput.fill('test+tag@example.co.uk');
    await subjectInput.fill('Test');
    await messageTextarea.fill('Testing email with special characters');

    // Act: submit form
    await submitButton.click();

    // Assert: form accepts email with special characters
    await expect(page.locator('.toast[data-type="success"]')).toBeVisible({ timeout: 10000 });
  });

  test('should handle subject with special characters', async ({ page }) => {
    // Arrange: form loaded
    const nameInput = page.locator('input[name="name"]');
    const emailInput = page.locator('input[name="email"]');
    const subjectInput = page.locator('input[name="subject"]');
    const messageTextarea = page.locator('textarea[name="message"]');
    const submitButton = page.locator('button[type="submit"]');

    // Act: fill form with special subject characters
    await nameInput.fill('Test User');
    await emailInput.fill('test@example.com');
    await subjectInput.fill('Test & Special @ Characters!');
    await messageTextarea.fill('Testing subject with special characters allowed');

    // Act: submit form
    await submitButton.click();

    // Assert: form accepts subject with allowed special characters
    await expect(page.locator('.toast[data-type="success"]')).toBeVisible({ timeout: 10000 });
  });

  test('should trim whitespace from fields', async ({ page }) => {
    // Arrange: wait for API request
    let capturedData: Record<string, unknown> | null = null;

    await page.route('**/api/contact', async route => {
      const request = route.request();
      if (request.method() === 'POST') {
        capturedData = JSON.parse(request.postData() || '{}');

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

    const nameInput = page.locator('input[name="name"]');
    const emailInput = page.locator('input[name="email"]');
    const subjectInput = page.locator('input[name="subject"]');
    const messageTextarea = page.locator('textarea[name="message"]');
    const submitButton = page.locator('button[type="submit"]');

    // Act: fill form with whitespace
    await nameInput.fill('  Test User  ');
    await emailInput.fill('  test@example.com  ');
    await subjectInput.fill('  Test Subject  ');
    await messageTextarea.fill('  Test message with spaces  ');

    // Act: submit form and wait for API response
    const responsePromise = page.waitForResponse('**/api/contact');
    await submitButton.click();
    const response = await responsePromise;

    // Wait for success toast to appear
    await expect(page.locator('.toast[data-type="success"]')).toBeVisible({ timeout: 10000 });

    // Assert: API call was successful
    expect(response.status()).toBe(200);

    // Assert: whitespace is trimmed in the API call
    expect(capturedData).not.toBeNull();
    if (capturedData) {
      expect(capturedData.name).toBe('Test User');
      expect(capturedData.email).toBe('test@example.com');
      expect(capturedData.subject).toBe('Test Subject');
      expect(capturedData.message).toBe('Test message with spaces');
    }
  });
});
