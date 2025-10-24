import { test, expect } from '@playwright/test';

test.describe('RSS Page', () => {
    test('should load initial articles', async ({ page }) => {
        // Accéder à la page RSS
        await page.goto('/rss');

        // Vérifier que le titre de la page est correct
        await expect(page.locator('h1')).toContainText('Mes flux RSS favoris');

        // Vérifier que la grille d'articles est présente
        const articlesGrid = page.locator('.articles-grid');
        await expect(articlesGrid).toBeVisible();

        // Vérifier qu'il y a 9 articles au chargement initial
        const articles = page.locator('.articles-grid article');
        await expect(articles).toHaveCount(9);

        // Vérifier que chaque article a les éléments requis
        const firstArticle = articles.first();
        await expect(firstArticle.locator('img')).toBeVisible();
        await expect(firstArticle.locator('h3')).toBeVisible();
        await expect(firstArticle.locator('a')).toBeVisible();
        await expect(firstArticle.locator('.text-sm')).toBeVisible(); // Date
    });

    test('should load more articles when clicking load more button', async ({ page }) => {
        await page.goto('/rss');

        // Compter le nombre initial d'articles
        const initialArticles = await page.locator('.articles-grid article').count();
        expect(initialArticles).toBe(9);

        // Cliquer sur le bouton "Charger plus"
        const loadMoreBtn = page.locator('#loadMore');
        await expect(loadMoreBtn).toBeVisible();
        await loadMoreBtn.click();

        // Attendre que les nouveaux articles soient chargés
        await page.waitForResponse(response =>
            response.url().includes('/api/rss') &&
            response.status() === 200
        );

        // Vérifier qu'il y a plus d'articles qu'avant
        const newArticleCount = await page.locator('.articles-grid article').count();
        expect(newArticleCount).toBe(18); // 9 articles initiaux + 9 nouveaux

        // Vérifier que les nouveaux articles sont correctement formatés
        const lastArticle = page.locator('.articles-grid article').last();
        await expect(lastArticle.locator('img')).toBeVisible();
        await expect(lastArticle.locator('h3')).toBeVisible();
        await expect(lastArticle.locator('a')).toBeVisible();
        await expect(lastArticle.locator('.text-sm')).toBeVisible(); // Date
    });

    test('should handle empty response gracefully', async ({ page }) => {
        await page.goto('/rss');

        // Intercepter les requêtes à l'API
        await page.route('**/api/rss**', async (route) => {
            const url = route.request().url();
            if (url.includes('page=1')) {
                // Première page : retourner des articles normaux
                await route.fulfill({
                    json: new Array(9).fill({
                        title: 'Test Article',
                        url: 'https://example.com',
                        pub_date: new Date().toISOString(),
                        description: 'Test Description',
                        image_url: 'https://placehold.co/600x400'
                    })
                });
            } else {
                // Pages suivantes : retourner un tableau vide
                await route.fulfill({ json: [] });
            }
        });

        // Cliquer sur le bouton "Charger plus"
        const loadMoreBtn = page.locator('#loadMore');
        await loadMoreBtn.click();

        // Vérifier que le bouton est désactivé et affiche le bon message
        await expect(loadMoreBtn).toBeDisabled();
        await expect(loadMoreBtn).toHaveText("Plus d'articles disponibles");
    });

    test('should handle API errors gracefully', async ({ page }) => {
        await page.goto('/rss');

        // Intercepter la prochaine requête à l'API et simuler une erreur
        await page.route('**/api/rss**', async (route) => {
            const url = route.request().url();
            if (url.includes('page=2')) {
                await route.fulfill({ status: 500 });
            } else {
                await route.continue();
            }
        });

        // Cliquer sur le bouton "Charger plus"
        const loadMoreBtn = page.locator('#loadMore');
        await loadMoreBtn.click();

        // Vérifier que le bouton affiche le message d'erreur
        await expect(loadMoreBtn).toHaveText('Erreur lors du chargement');
    });
});
