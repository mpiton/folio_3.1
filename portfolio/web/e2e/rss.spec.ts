import { test, expect } from '@playwright/test';

declare global {
  interface Window {
    ENV: {
      MODE: string;
    };
    ToastManager: unknown;
  }
}

test.describe('RSS Feed', () => {
  test.beforeEach(async ({ page }) => {
    // Set test mode
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

    // Mock ToastManager
    await page.addInitScript(() => {
      class MockToastManager {
        private element: HTMLElement;
        private static activeToasts: HTMLElement[] = [];

        constructor(element: HTMLElement) {
          this.element = element;
        }

        show() {
          // Clone the toast if needed
          if (!this.element.classList.contains('toast--cloned')) {
            const clone = this.element.cloneNode(true) as HTMLElement;
            clone.classList.add('toast--cloned');
            document.body.appendChild(clone);
            this.element = clone;
          }

          // Add to active toasts and update position
          MockToastManager.activeToasts.push(this.element);
          this.updatePosition();

          // Make visible
          this.element.classList.add('toast--visible');
        }

        close() {
          this.element.classList.remove('toast--visible');
          const index = MockToastManager.activeToasts.indexOf(this.element);
          if (index !== -1) {
            MockToastManager.activeToasts.splice(index, 1);
          }
          if (this.element.parentNode) {
            this.element.parentNode.removeChild(this.element);
          }
        }

        private updatePosition() {
          const index = MockToastManager.activeToasts.indexOf(this.element);
          if (index !== -1) {
            const offset = index * 16;
            this.element.style.setProperty('--toast-offset', `${offset}px`);
          }
        }
      }

      window.ToastManager = MockToastManager;
    });

    await page.goto('/rss');

    await Promise.all([
      page.waitForLoadState('networkidle', { timeout: 60000 }),
      page.waitForLoadState('domcontentloaded'),
    ]);
  });

  test('should display RSS articles and handle load more', async ({ page }) => {
    // Navigate to RSS page
    await page.goto('/rss');
    await Promise.all([
      page.waitForLoadState('networkidle'),
      page.waitForLoadState('domcontentloaded'),
    ]);

    // Check initial articles are visible
    const articles = page.locator('.rss-article');
    await expect(articles).toHaveCount(2);
    await expect(articles.first()).toBeVisible();
    await expect(articles.first()).toContainText('Test Article 1');

    // Click load more and verify new articles appear
    await page.click('button:has-text("Charger plus")');
    await expect(articles).toHaveCount(3);
    await expect(articles.nth(2)).toBeVisible();
  });

  test('should handle RSS API error', async ({ page }) => {
    // Mock API error
    await page.route('**/api/rss', route =>
      route.fulfill({
        status: 500,
        contentType: 'application/json',
        body: JSON.stringify({ error: 'Internal Server Error' }),
      })
    );

    // Reload page to trigger error
    await page.reload();

    // Verify error toast
    const errorToast = page.locator('.toast--error.toast--cloned');
    await expect(errorToast).toBeVisible({ timeout: 15000 });
    await expect(errorToast).toContainText('Internal Server Error');
  });
});
