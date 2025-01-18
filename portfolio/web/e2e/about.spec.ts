import { test, expect } from '@playwright/test';

test.describe('About Page', () => {
  test.beforeEach(async ({ page }) => {
    // Augmenter le timeout et ajouter des retries pour la navigation
    await test.step('Navigate to about page', async () => {
      await page.goto('/about', {
        timeout: 30000,
        waitUntil: 'networkidle'
      });
    });
  });

  test('should display all required sections', async ({ page }) => {
    // Vérifier le titre de la page
    await expect(page).toHaveTitle(/À propos/);

    // Vérifier la section d'introduction
    const introSection = page.locator('.about-intro');
    await expect(introSection.locator('h1')).toHaveText('À propos de moi');
    await expect(introSection.locator('img')).toBeVisible();

    // Vérifier la section des compétences
    const skillsSection = page.locator('.skills');
    await expect(skillsSection.locator('h2')).toHaveText('Mes compétences');

    // Vérifier les catégories de compétences
    const skillCategories = skillsSection.locator('.skill-category');
    await expect(skillCategories).toHaveCount(3);

    // Vérifier les titres des catégories
    const categoryTitles = await skillCategories.locator('h3').allTextContents();
    expect(categoryTitles).toEqual([
      'Langages',
      'Frameworks Frontend & Hybrids',
      'Frameworks Backend'
    ]);

    // Vérifier les compétences de la première catégorie (Langages)
    const languageSkills = skillCategories.first().locator('.skill-name');
    const languageNames = await languageSkills.allTextContents();
    expect(languageNames).toContain('HTML/CSS');
    expect(languageNames).toContain('JS (JavaScript)');
    expect(languageNames).toContain('PHP');
    expect(languageNames).toContain('Python');
    expect(languageNames).toContain('Dart');
    expect(languageNames).toContain('Rust');
    expect(languageNames).toContain('Go');

    // Vérifier que chaque compétence a une barre de progression
    for (const category of await skillCategories.all()) {
      const skills = await category.locator('.skill').all();
      for (const skill of skills) {
        await expect(skill.locator('.skill-bar')).toBeVisible();
        await expect(skill.locator('.skill-level')).toBeVisible();
      }
    }
  });

  test('should have working navigation', async ({ page }) => {
    // Vérifier que la navigation est présente
    const nav = page.locator('nav');
    await expect(nav).toBeVisible();

    // Vérifier la présence du lien de contact dans la navigation
    const contactLink = nav.locator('a[href="/contact"]');
    await expect(contactLink).toBeVisible();
  });
});

// Configurer les retries au niveau du test
test.describe.configure({ retries: 2 });
