import { test, expect } from '@playwright/test';

test.describe('RSS Page', () => {
    test.beforeEach(async ({ page }) => {
        // Configuration du mock pour toutes les requêtes API
        await page.route('**/api/rss**', async (route) => {
            const url = route.request().url();
            const params = new URLSearchParams(new URL(url).search);
            const pageNum = parseInt(params.get('page') || '1');

            // Si c'est la première page ou si on demande plus d'articles
            if (pageNum <= 2) {
                await route.fulfill({
                    status: 200,
                    contentType: 'application/json',
                    body: JSON.stringify(Array(9).fill({
                        title: 'Test Article',
                        url: 'https://example.com',
                        pub_date: new Date().toISOString(),
                        description: 'Test Description',
                        image_url: 'https://placehold.co/600x400'
                    }))
                });
            } else {
                // Pour les pages suivantes, retourner un tableau vide
                await route.fulfill({
                    status: 200,
                    contentType: 'application/json',
                    body: JSON.stringify([])
                });
            }
        });

        // Attendre que le serveur soit prêt avant de naviguer
        await test.step('Wait for server and navigate', async () => {
            let retries = 0;
            while (retries < 3) {
                try {
                    await page.goto('/rss', {
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

    test('should load initial articles', async ({ page }) => {
        // Attendre que la page soit complètement chargée
        await page.waitForLoadState('networkidle');
        await page.waitForTimeout(2000);

        // Vérifier que le titre de la page est correct
        const title = page.getByRole('heading', { name: 'Mes flux RSS favoris' });
        await expect(title).toBeVisible({ timeout: 30000 });

        // Vérifier qu'il y a 9 articles avec un timeout plus long
        const articles = page.locator('.articles-grid article');
        await expect(articles).toHaveCount(9, { timeout: 30000 });

        // Vérifier que chaque article a les éléments requis avec un timeout plus long
        const firstArticle = articles.first();
        await expect(firstArticle.locator('img')).toBeVisible({ timeout: 30000 });
        await expect(firstArticle.locator('h3')).toBeVisible({ timeout: 30000 });
        await expect(firstArticle.locator('a')).toBeVisible({ timeout: 30000 });
        await expect(firstArticle.locator('.text-sm')).toBeVisible({ timeout: 30000 });
    });

    test('should load more articles when clicking load more button', async ({ page }) => {
        // Attendre que la page soit complètement chargée
        await page.waitForLoadState('networkidle');
        await page.waitForTimeout(2000);

        // Compter le nombre initial d'articles avec un timeout plus long
        const articles = page.locator('.articles-grid article');
        await expect(articles).toHaveCount(9, { timeout: 30000 });

        // Cliquer sur le bouton "Charger plus"
        const loadMoreBtn = page.locator('#loadMore');
        await expect(loadMoreBtn).toBeVisible({ timeout: 30000 });
        await loadMoreBtn.click();

        // Attendre que les nouveaux articles soient chargés avec un timeout plus long
        await expect(articles).toHaveCount(18, { timeout: 30000 });

        // Vérifier que les nouveaux articles sont correctement formatés avec un timeout plus long
        const lastArticle = articles.last();
        await expect(lastArticle.locator('img')).toBeVisible({ timeout: 30000 });
        await expect(lastArticle.locator('h3')).toBeVisible({ timeout: 30000 });
        await expect(lastArticle.locator('a')).toBeVisible({ timeout: 30000 });
        await expect(lastArticle.locator('.text-sm')).toBeVisible({ timeout: 30000 });
    });

    test('should handle empty response gracefully', async ({ page }) => {
        // Compter le nombre initial d'articles
        const articles = page.locator('.articles-grid article');
        await expect(articles).toHaveCount(9);

        // Cliquer sur le bouton "Charger plus" deux fois pour atteindre la page vide
        const loadMoreBtn = page.locator('#loadMore');
        await loadMoreBtn.click();
        await expect(articles).toHaveCount(18);

        await loadMoreBtn.click();

        // Vérifier que le bouton est désactivé et affiche le bon message
        await expect(loadMoreBtn).toBeDisabled();
        await expect(loadMoreBtn).toHaveText("Plus d'articles disponibles");
    });

    test('should handle API errors gracefully', async ({ page }) => {
        // Override le mock pour simuler une erreur
        await page.route('**/api/rss**', async (route) => {
            const url = route.request().url();
            if (url.includes('page=3')) {
                await route.fulfill({ status: 500 });
            } else {
                await route.fulfill({
                    status: 200,
                    contentType: 'application/json',
                    body: JSON.stringify(Array(9).fill({
                        title: 'Test Article',
                        url: 'https://example.com',
                        pub_date: new Date().toISOString(),
                        description: 'Test Description',
                        image_url: 'https://placehold.co/600x400'
                    }))
                });
            }
        });

        await page.reload();
        await page.waitForLoadState('networkidle');

        // Vérifier le nombre initial d'articles
        const articles = page.locator('.articles-grid article');
        await expect(articles).toHaveCount(9);

        // Cliquer sur le bouton "Charger plus"
        const loadMoreBtn = page.locator('#loadMore');
        await loadMoreBtn.click();
        await expect(articles).toHaveCount(18);

        // Cliquer à nouveau pour déclencher l'erreur
        await loadMoreBtn.click();

        // Vérifier que le bouton affiche le message d'erreur
        await expect(loadMoreBtn).toHaveText('Erreur lors du chargement');
    });
});
