# Plan de Développement Frontend - Portfolio v3.1

## 0. Conventions & Standards

### Conventions de Codage Astro

1.  **Nommage des fichiers**
    -   Composants (`.astro`): `PascalCase` (ex: `Header.astro`)
    -   Scripts (`.ts`): `PascalCase` pour les classes, `camelCase` pour les fonctions/variables (ex: `ToastManager.ts`)
    -   Pages (`.astro`): `kebab-case` (ex: `mentions-legales.astro`)

2.  **Documentation**
    -   Les commentaires dans le code sont rédigés en français.
    -   L'organisation du code au sein des fichiers `.astro` suit une structure logique : imports, props, logique serveur, template HTML, et enfin le style.

3.  **Tests**
    -   Les tests End-to-End (E2E) sont écrits avec Playwright et se trouvent dans le dossier `e2e/`.
    -   Il n'y a pas de tests unitaires (type Vitest) actuellement configurés pour le frontend.

### Conventions Git

1.  **Branches**
    -   `main` : Production.
    -   `feature/[description]` : Développement de nouvelles fonctionnalités.
    -   `fix/[description]` : Correction de bugs.

2.  **Commits**
    -   Format : `type(scope): description` (ex: `feat(contact): ajoute la validation du formulaire`).
    -   Types : `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`.

## 1. Stack Technique et Configuration

Le frontend est construit avec **Astro** et **TypeScript**. Le style est géré par **Tailwind CSS**.

### Dépendances principales
-   `astro`
-   `@astrojs/tailwind`
-   `@astrojs/sitemap`
-   `@vite-pwa/astro` (pour la fonctionnalité PWA)
-   `playwright` et `@playwright/test` pour les tests E2E.

### Structure du projet
```
portfolio/web/
├── src/
│   ├── components/
│   │   ├── common/       # Composants réutilisables (Button, Card, etc.)
│   │   ├── layout/       # Composants de mise en page (Header, Footer)
│   │   └── sections/     # Sections de page (Hero, ContactForm)
│   ├── layouts/
│   │   └── Layout.astro  # Layout principal
│   ├── pages/            # Les pages du site
│   ├── assets/           # Images et autres ressources statiques
│   └── scripts/          # Scripts TypeScript (ex: ToastManager)
├── public/               # Fichiers statiques (favicon, etc.)
├── e2e/                  # Tests E2E Playwright
├── astro.config.mjs      # Configuration d'Astro
└── package.json
```

### Configuration d'Astro (`astro.config.mjs`)
```javascript
import { defineConfig } from 'astro/config';
import tailwind from '@astrojs/tailwind';
import sitemap from '@astrojs/sitemap';
import AstroPWA from '@vite-pwa/astro';

export default defineConfig({
  site: 'https://mathieu-piton.com',
  output: 'static',
  integrations: [
    tailwind(), 
    sitemap(),
    AstroPWA({ /* ... configuration PWA ... */ })
  ],
  vite: {
    // Optimisations de build...
  }
});
```

### Configuration des Tests (`playwright.config.ts`)
```javascript
import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './e2e',
  fullyParallel: true,
  reporter: 'html',
  use: {
    baseURL: 'http://localhost:4321',
    trace: 'on-first-retry',
  },
  projects: [
    { name: 'chromium', use: { ...devices['Desktop Chrome'] } },
  ],
  webServer: {
    command: 'npm run dev',
    url: 'http://localhost:4321',
    reuseExistingServer: !process.env.CI,
  },
});
```

## 2. État Actuel du Développement

### Composants et Pages
-   **Pages fonctionnelles** : Accueil, À propos, Contact, Mentions Légales, Flux RSS.
-   **Composants principaux** : `Header`, `Footer`, `Hero`, `ContactForm`, `RssFeeds`.
-   **Composants communs** : `Button`, `Card`, `Input`, `Toast` sont implémentés et utilisés. D'autres (`Modal`, `Loader`, `Pagination`) existent mais ne sont utilisés que sur une page de test (`/components`).

### Tests
-   Les tests E2E couvrent les fonctionnalités critiques :
    -   Navigation entre les pages.
    -   Soumission du formulaire de contact.
    -   Affichage des flux RSS.
    -   Rendu des composants sur la page de test.

### Optimisations et Sécurité
-   **Performance** : Optimisation des images et build statique pour de bonnes performances.
-   **PWA** : Le site est configurable comme une Progressive Web App.
-   **Sécurité** : Les headers de sécurité (CSP, XSS-Protection) sont gérés via le fichier `public/_headers`.

## 3. Axes d'amélioration futurs
-   **Internationalisation (i18n)** : Implémenter le support multilingue, une fonctionnalité initialement prévue mais non réalisée.
-   **Tests unitaires** : Ajouter des tests unitaires pour les composants (par ex. avec `@testing-library/astro`) pour compléter les tests E2E.
-   **Nettoyage** : Supprimer les composants de la page de test (`/components`) s'ils ne sont pas destinés à être utilisés dans l'application finale.
