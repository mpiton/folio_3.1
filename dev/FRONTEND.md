# Plan de Développement Frontend - Portfolio v3.1

## 0. Conventions & Standards

### Conventions de Codage Astro
1. **Nommage**
   - Composants : `PascalCase` (ex: `Header.astro`)
   - Scripts : `camelCase` (ex: `useTheme.ts`)
   - Styles : `kebab-case` (ex: `main-styles.css`)
   - Variables : `camelCase` (ex: `const userData`)
   - Constantes : `SCREAMING_SNAKE_CASE` (ex: `const MAX_ITEMS = 100`)

2. **Documentation**
   - Documentation des composants avec JSDoc
   - Documentation des fonctions avec commentaires explicatifs
   - Exemples de code dans la documentation
   - Commentaires en français pour la cohérence
   - Tests comme documentation vivante

3. **Organisation du Code**
   ```astro
   ---
   // 1. Imports
   import { Component } from '@/components';

   // 2. Props et Types
   interface Props {
     title: string;
   }

   // 3. Data Fetching et Logic
   const { title } = Astro.props;
   const data = await fetchData();
   ---

   <!-- 4. Template -->
   <Component>
     <h1>{title}</h1>
   </Component>

   <style>
   /* 5. Styles */
   </style>
   ```

4. **Tests**
   ```typescript
   // tests/components/Header.test.ts
   import { describe, it, expect } from 'vitest';
   import { render } from '@testing-library/astro';
   import Header from '@/components/Header.astro';

   describe('Header', () => {
     it('renders correctly', async () => {
       const { getByText } = await render(Header);
       expect(getByText('Title')).toBeInTheDocument();
     });
   });
   ```

5. **Gestion des Erreurs**
   - Utilisation de try/catch
   - Messages d'erreur descriptifs en français
   - Pages d'erreur personnalisées (404, 500)
   - Gestion des erreurs côté serveur

### Conventions Git
1. **Branches**
   - `main` : production
   - `develop` : développement
   - `feature/nom-feature` : nouvelles fonctionnalités
   - `fix/nom-fix` : corrections de bugs

2. **Commits**
   - Format : `type(scope): description`
   - Types : feat, fix, docs, style, refactor, test, chore
   - Description en français
   - Exemple : `feat(auth): ajoute la validation du formulaire`

## 1. Charte Graphique

### Couleurs
```css
:root {
    --primary-color: #578E7E;    /* Vert principal */
    --secondary-color: #F5ECD5;  /* Beige clair */
    --accent-color: #FFFAEC;    /* Blanc cassé */
    --text-color: #3D3D3D;      /* Gris foncé */
}
```

### Typographie
```css
/* Titres */
--font-heading: 'Poppins', sans-serif;
/* Corps de texte */
--font-body: 'Open Sans', sans-serif;
```

### Composants UI
1. **Boutons**
   ```astro
   ---
   interface Props {
     variant?: 'primary' | 'secondary';
     size?: 'sm' | 'md' | 'lg';
   }
   ---
   <button class:list={['btn', variant, size]}>
     <slot />
   </button>

   <style>
     .btn {
       background-color: var(--primary-color);
       color: var(--accent-color);
       padding: 10px 20px;
       border-radius: 5px;
       transition: background-color 0.3s;
     }
   </style>
   ```

2. **Cartes**
   ```astro
   <div class="card">
     <slot />
   </div>

   <style>
     .card {
       background-color: var(--accent-color);
       border-radius: 10px;
       box-shadow: 0 0 20px rgba(0,0,0,0.1);
       padding: 40px;
     }
   </style>
   ```

### Responsive Design
```css
/* Breakpoints */
--mobile: 576px;
--tablet: 768px;
--desktop: 1024px;
--large: 1200px;

/* Media queries */
@media (max-width: var(--tablet)) {
    /* Styles tablette */
}

@media (max-width: var(--mobile)) {
    /* Styles mobile */
}
```

## 2. Structure du Projet
```
portfolio/web/
├── src/
│   ├── components/
│   │   ├── common/
│   │   │   ├── Button.astro
│   │   │   ├── Card.astro
│   │   │   └── Input.astro
│   │   ├── layout/
│   │   │   ├── Header.astro
│   │   │   └── Footer.astro
│   │   └── sections/
│   │       ├── Hero.astro
│   │       └── Projects.astro
│   ├── layouts/
│   │   ├── Layout.astro
│   │   └── BlogPost.astro
│   ├── pages/
│   │   ├── index.astro
│   │   ├── about.astro
│   │   ├── contact.astro
│   │   └── rss.xml.js
│   ├── content/
│   │   ├── blog/
│   │   └── projects/
│   ├── i18n/
│   │   ├── en.json
│   │   └── fr.json
│   ├── styles/
│   │   ├── global.css
│   │   └── utils.css
│   └── utils/
│       ├── seo.ts
│       └── api.ts
├── public/
│   ├── favicon.svg
│   ├── robots.txt
│   └── sitemap.xml
└── astro.config.mjs
```

## 3. Configuration Initiale

```bash
# Création du projet
npm create astro@latest portfolio-v3

# Installation des dépendances essentielles
npm install @astrojs/mdx
npm install @astrojs/sitemap
npm install @astrojs/tailwind
npm install astro-i18next
npm install sharp
npm install three @types/three
npm install @vite-pwa/astro workbox-window
```

### Configuration Astro
```javascript
// astro.config.mjs
import { defineConfig } from 'astro/config';
import mdx from '@astrojs/mdx';
import sitemap from '@astrojs/sitemap';
import tailwind from '@astrojs/tailwind';
import AstroPWA from '@vite-pwa/astro';
import pwaConfig from './src/pwa';

export default defineConfig({
  site: 'https://mathieu-piton.com',
  output: 'static',
  integrations: [
    mdx(),
    sitemap(),
    tailwind(),
    AstroPWA(pwaConfig)
  ],
  i18n: {
    defaultLocale: 'fr',
    locales: ['fr', 'en'],
  },
});
```

### Configuration PWA
```typescript
// src/pwa.ts
export default {
  registerType: 'autoUpdate',
  manifest: {
    name: 'Mathieu Piton - Portfolio',
    short_name: 'MP Portfolio',
    description: 'Portfolio de Mathieu Piton, développeur Full Stack',
    theme_color: '#578E7E',
    background_color: '#F5ECD5',
    display: 'standalone',
    icons: [/* ... */],
    start_url: '/',
    scope: '/'
  },
  workbox: {
    navigateFallback: process.env.NODE_ENV === 'production' ? '/404' : null,
    globPatterns: process.env.NODE_ENV === 'production'
      ? ['**/*.{css,js,html,svg,png,jpg,jpeg,gif,webp,woff,woff2,ttf,eot,ico}']
      : [],
    runtimeCaching: [
      {
        urlPattern: /^https:\/\/api\.mathieu-piton\.com\/.*$/,
        handler: 'NetworkFirst',
        options: {
          cacheName: 'api-cache',
          networkTimeoutSeconds: 5,
          expiration: {
            maxEntries: 50,
            maxAgeSeconds: 60 * 60 * 24
          }
        }
      }
    ]
  }
};
```

### Gestion des Assets PWA
```javascript
// scripts/generate-pwa-assets.js
import { promises as fs } from 'fs';
import sharp from 'sharp';
import path from 'path';

const ICONS_SIZES = [192, 512];
const SOURCE_ICON = 'public/favicon.svg';
const OUTPUT_DIR = 'public';

async function generatePWAIcons() {
  // Génération des icônes et du manifest
}
```

### Scripts de Build
```json
{
  "scripts": {
    "generate-pwa": "node scripts/generate-pwa-assets.js",
    "build": "npm run generate-pwa && astro build"
  }
}
```

### Fichiers à ignorer
```gitignore
# PWA
dev-dist/
public/manifest.webmanifest
public/icon-*.png
```

## 4. Phases de Développement

### Phase 1: Configuration & Infrastructure
- [x] Setup du projet Astro
- [x] Configuration des intégrations
- [x] Mise en place du système i18n
- [ ] Configuration du SEO
- [x] Setup des tests
- [x] Configuration de Playwright pour les tests E2E

### Phase 2: Composants de Base
- [x] Layout principal avec effets visuels
- [x] Composants communs
  - [x] Button (primary/outline variants)
  - [x] Card (avec variants et hover effects)
  - [x] Input (avec validation et gestion des erreurs)
  - [x] Toast (avec animations et gestion du temps)
  - [x] Modal (avec différentes tailles et gestion des clics)
- [x] Navigation responsive
- [x] Footer avec effet de flou et liens sociaux

### Phase 3: Pages Principales
- [x] Page d'accueil avec Hero section
- [x] Page À propos
- [x] Page Contact avec formulaire et validation
- [x] Flux RSS

### Phase 4: Optimisation
- [x] Performance de base
- [x] Tests E2E robustes
- [x] Gestion des erreurs
- [x] Accessibilité
- [x] SEO avancé
- [x] Analytics
- [x] Sécurité
  - [x] Validation des formulaires côté client
  - [x] Protection XSS
  - [x] Gestion des erreurs API
  - [x] Headers de sécurité
  - [x] CSP configuration
  - [x] Rate limiting handling

### Phase 5: Intégration API
- [x] Configuration des endpoints
  - [x] Contact form `/api/contact`
  - [x] RSS feed `/api/rss`
  - [x] Health check `/health`
- [x] Gestion des erreurs API
  - [x] Timeouts
  - [x] Rate limiting
  - [x] Validation errors
  - [x] Network errors
- [x] Feedback utilisateur
  - [x] Loading states
  - [x] Error messages
  - [x] Success notifications
  - [x] Rate limit warnings

### Phase 6: Tests
- [x] Tests E2E API integration
  ```typescript
  // tests/api/contact.spec.ts
  test('should handle rate limiting', async ({ page }) => {
    // Simulate multiple rapid requests
    for (let i = 0; i < 5; i++) {
      await page.click('#submit-contact');
    }

    // Check for rate limit warning
    await expect(page.locator('.toast--error')).toContainText('Trop de requêtes');
  });
  ```

- [x] Tests de sécurité
  ```typescript
  // tests/security/xss.spec.ts
  test('should sanitize user input', async ({ page }) => {
    await page.fill('#message', '<script>alert("xss")</script>');
    await page.click('#submit');

    // Verify sanitized output
    const content = await page.textContent('.message-content');
    expect(content).not.toContain('<script>');
  });
  ```

## 5. Tests

### Tests E2E avec Playwright
```typescript
// Configuration améliorée
test.beforeEach(async ({ page }) => {
  // Attente du serveur et navigation avec retry
  await page.goto('/components', {
    waitUntil: 'networkidle',
    timeout: 30000
  });

  // Attente de la stabilité de la page
  await page.waitForLoadState('domcontentloaded');
});

// Tests des composants
test('should show and auto-dismiss toast', async ({ page }) => {
  // Attente de la stabilité des boutons
  const button = page.getByRole('button', { name: 'Show Success Toast' });
  await button.waitFor({ state: 'visible', timeout: 15000 });
  await button.click();

  // Vérification du toast
  const toast = page.locator('.toast--success.toast--cloned');
  await toast.waitFor({ state: 'visible', timeout: 15000 });
  await expect(toast).toHaveClass(/toast--visible/);
});
```

### Configuration Playwright
```javascript
// playwright.config.ts
import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'html',
  use: {
    baseURL: 'http://localhost:4321',
    trace: 'on-first-retry',
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
    {
      name: 'firefox',
      use: { ...devices['Desktop Firefox'] },
    },
    {
      name: 'webkit',
      use: { ...devices['Desktop Safari'] },
    },
  ],
  webServer: {
    command: 'npm run dev',
    port: 4321,
    reuseExistingServer: !process.env.CI,
  },
});
```

## 6. Implémentation SEO

### Configuration de Base
```typescript
// src/utils/seo.ts
export const defaultSEO = {
  title: 'Mathieu Piton - Développeur Full Stack',
  description: 'Portfolio de Mathieu Piton, développeur Full Stack spécialisé en Rust et TypeScript',
  openGraph: {
    type: 'website',
    locale: 'fr_FR',
    url: 'https://mathieupiton.fr',
    site_name: 'Mathieu Piton',
  },
};
```

### Utilisation dans les Pages
```astro
---
import { defaultSEO } from '../utils/seo';
---
<head>
  <title>{defaultSEO.title}</title>
  <meta name="description" content={defaultSEO.description} />
  <meta property="og:title" content={defaultSEO.title} />
  <!-- ... autres meta tags ... -->
</head>
```

## 7. Internationalisation

### Configuration i18n
```typescript
// astro-i18next.config.ts
export default {
  defaultLocale: 'fr',
  locales: ['fr', 'en'],
  namespaces: ['common', 'home', 'about'],
  defaultNamespace: 'common',
};
```

### Utilisation
```astro
---
import { t } from 'astro-i18next';
---
<h1>{t('home.title')}</h1>
```

## 8. Tests et Qualité

### Configuration des Tests
```typescript
// vitest.config.ts
import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    environment: 'jsdom',
    setupFiles: ['./tests/setup.ts'],
    include: ['src/**/*.test.{ts,tsx}'],
  },
});
```

### Exemples de Tests
```typescript
// Tests de composants
import { render } from '@testing-library/astro';
import MyComponent from '../src/components/MyComponent.astro';

test('MyComponent renders correctly', async () => {
  const { getByText } = await render(MyComponent);
  expect(getByText('Hello')).toBeInTheDocument();
});
```

## 9. Déploiement

### Configuration CI/CD
```yaml
# .github/workflows/deploy.yml
name: Deploy
on:
  push:
    branches: [ main ]
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install
        run: npm ci
      - name: Build
        run: npm run build
      - name: Deploy
        uses: cloudflare/wrangler-action@2.0.0
```

## 10. Métriques de Réussite

- [ ] Lighthouse score > 95
- [ ] WCAG 2.1 AA compliance
- [ ] Tests coverage > 80%
- [ ] < 1s initial load time
- [ ] < 50ms TTI
- [ ] Perfect accessibility score
- [ ] SEO score > 95

## 11. Directives de Développement

1. **Performance First**
   - Utilisation d'images optimisées
   - Code splitting automatique
   - Prefetching intelligent
   - Minification des assets

2. **Accessibilité**
   - ARIA labels
   - Contraste des couleurs
   - Navigation au clavier
   - Support lecteur d'écran

3. **SEO**
   - Meta tags dynamiques
   - Sitemap automatique
   - Structured data
   - Canonical URLs

4. **Maintenance**
   - Documentation claire
   - Tests automatisés
   - Code review systématique
   - Monitoring des performances
