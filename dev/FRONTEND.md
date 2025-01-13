# Plan de Développement Frontend - Portfolio v3.1

## 0. Conventions & Standards

### Conventions de Codage Rust
1. **Nommage**
   - Modules : `snake_case` (ex: `mod auth_service`)
   - Types/Structs/Enums : `PascalCase` (ex: `struct UserProfile`)
   - Fonctions/Variables : `snake_case` (ex: `fn validate_input()`)
   - Constantes : `SCREAMING_SNAKE_CASE` (ex: `const MAX_ITEMS: u32 = 100`)
   - Composants Dioxus : `PascalCase` (ex: `fn HeaderComponent()`)

2. **Documentation**
   - Documentation des modules avec `//! Module-level documentation`
   - Documentation des fonctions/types avec `/// Function/Type documentation`
   - Exemples de code dans la documentation avec ```rust
   - Commentaires en français pour la cohérence
   - Tests comme documentation vivante

3. **Organisation du Code**
   ```rust
   // Ordre des imports
   use std::*;               // Imports standard
   use external::*;          // Imports externes
   use crate::*;            // Imports locaux

   // Structure des composants
   #[component]
   pub fn ComponentName(cx: Scope) -> Element {
       // 1. Hooks et états
       let state = use_state(cx, || initial_value);

       // 2. Callbacks et gestionnaires d'événements
       let on_click = move |_| { /* ... */ };

       // 3. Logique de rendu
       render! {
           div { /* ... */ }
       }
   }
   ```

4. **Tests**
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       // Tests unitaires
       #[test]
       fn test_component_behavior() {
           // Arrange
           // Act
           // Assert
       }

       // Tests d'intégration
       #[test]
       fn test_component_integration() {
           // ...
       }
   }
   ```

5. **Gestion des Erreurs**
   - Utilisation de `Result` et `Option`
   - Messages d'erreur descriptifs en français
   - Propagation avec l'opérateur `?`
   - Types d'erreur personnalisés quand nécessaire

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
```scss
:root {
    --primary-color: #578E7E;    // Vert principal
    --secondary-color: #F5ECD5;  // Beige clair
    --accent-color: #FFFAEC;    // Blanc cassé
    --text-color: #3D3D3D;      // Gris foncé
}
```

### Typographie
```scss
// Titres
font-family: 'Poppins', sans-serif;
// Corps de texte
font-family: 'Open Sans', sans-serif;
```

### Composants UI
1. **Boutons**
   ```scss
   .btn {
       background-color: var(--primary-color);
       color: var(--accent-color);
       padding: 10px 20px;
       border-radius: 5px;
       transition: background-color 0.3s;
   }
   ```

2. **Cartes**
   ```scss
   .card {
       background-color: var(--accent-color);
       border-radius: 10px;
       box-shadow: 0 0 20px rgba(0,0,0,0.1);
       padding: 40px;
   }
   ```

3. **Animations**
   ```scss
   // Transitions
   transition: all 0.3s ease;

   // Hover effects
   &:hover {
       transform: translateY(-2px);
       box-shadow: 0 6px 10px rgba(0, 0, 0, 0.2);
   }
   ```

### Responsive Design
```scss
// Breakpoints
$mobile: 576px;
$tablet: 768px;
$desktop: 1024px;
$large: 1200px;

// Media queries
@media (max-width: $tablet) {
    // Styles tablette
}

@media (max-width: $mobile) {
    // Styles mobile
}
```

## 1. Configuration Initiale & Architecture

### Configuration du Projet Existant
```bash
# Se placer dans le dossier web existant
cd portfolio/web

# Ajout des dépendances essentielles
cargo add dioxus
cargo add dioxus-web
cargo add dioxus-router
cargo add dioxus-hooks
cargo add web-sys
cargo add wasm-bindgen
cargo add gloo
cargo add serde
cargo add serde_json
cargo add reqwest
cargo add i18n-embed
cargo add rust-embed
```

### Structure du Projet Existante
```
portfolio/web/
├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── components/
│   │   ├── mod.rs
│   │   ├── header/
│   │   ├── footer/
│   │   ├── about/
│   │   ├── contact/
│   │   ├── rss/
│   │   └── common/
│   ├── pages/
│   │   ├── mod.rs
│   │   ├── home.rs
│   │   ├── about.rs
│   │   ├── contact.rs
│   │   └── rss.rs
│   ├── hooks/
│   │   ├── mod.rs
│   │   ├── use_i18n.rs
│   │   └── use_theme.rs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── api.rs
│   │   └── storage.rs
│   ├── utils/
│   │   ├── mod.rs
│   │   └── seo.rs
│   ├── i18n/
│   │   ├── en.json
│   │   └── fr.json
│   └── styles/
│       ├── main.scss
│       └── components/
├── public/
│   ├── index.html
│   ├── robots.txt
│   └── sitemap.xml
├── tests/
|    ├── components/
|    └── pages/
├── static/
│   ├── styles/
│   │   ├── main.scss
│   │   └── components/
│   ├── images/
│   ├── fonts/
│   └── locales/
├── tests/
│   ├── composants/
│   └── pages/
└── Cargo.toml
```

## 2. Phases de Développement (Approche TDD)

### Phase 1: Configuration de Base & Infrastructure de Test

1. **Configuration de Base**
   - [ ] Configuration de Dioxus avec WASM
   - [ ] Mise en place de l'environnement de test
   - [ ] Configuration du traitement SCSS
   - [ ] Configuration du système i18n
   - [ ] Configuration du routage

2. **Infrastructure de Test**
   ```rust
   // Structure de test exemple
   #[cfg(test)]
   mod tests {
       use super::*;
       use dioxus::test::*;

       #[test]
       fn test_component_renders() {
           let mut test = TestRunner::new();
           test.run(|cx| {
               render! { Component {} }
           });
           // Assertions
       }
   }
   ```

### Phase 2: Développement des Composants

1. **Composants Communs**
   - [ ] Bouton
   ```rust
   #[test]
   fn test_button_click() {
       // Test button interactions
   }
   ```
   - [ ] Input
   - [ ] Card
   - [ ] Modal
   - [ ] Loading Spinner
   - [ ] Error Boundary

2. **Layout Components**
   - [ ] Header
   - [ ] Footer
   - [ ] Navigation
   - [ ] Language Switcher

### Phase 3: Composants des Pages

1. **Page d'Accueil**
   - [ ] Section héro
   - [ ] Section compétences
   - [ ] Grille des projets
   - [ ] Balises SEO

2. **Page À Propos**
   - [ ] Section biographie
   - [ ] Chronologie d'expérience
   - [ ] Matrice de compétences
   - [ ] Certifications

3. **Page Flux RSS**
   - [ ] Grille des flux
   - [ ] Système de filtrage
   - [ ] Fonctionnalité de recherche
   - [ ] Pagination

4. **Page Contact**
   - [ ] Formulaire de contact
   - [ ] Validation du formulaire
   - [ ] États succès/erreur
   - [ ] Intégration anti-spam

## 3. Implémentation de l'Accessibilité

### Approche de Test
```rust
#[test]
fn test_accessibility_attributes() {
    // Test ARIA attributes
    // Test keyboard navigation
    // Test screen reader compatibility
}
```

### Checklist
- [ ] ARIA labels
- [ ] Keyboard navigation
- [ ] Color contrast
- [ ] Focus management
- [ ] Screen reader support
- [ ] Skip links
- [ ] Form labels
- [ ] Error announcements

## 4. Implémentation SEO

### Structure des Balises Meta
```html
<head>
    <title>%PAGE_TITLE% | Mathieu Piton</title>
    <meta name="description" content="%PAGE_DESCRIPTION%">
    <meta name="keywords" content="%PAGE_KEYWORDS%">
    <meta property="og:title" content="%PAGE_TITLE%">
    <meta property="og:description" content="%PAGE_DESCRIPTION%">
    <meta property="og:image" content="%PAGE_IMAGE%">
    <link rel="canonical" href="%PAGE_URL%">
    <meta name="robots" content="index, follow">
</head>
```

### Implementation Tasks
- [ ] Dynamic meta tags
- [ ] Structured data
- [ ] Sitemap generation
- [ ] robots.txt configuration
- [ ] Canonical URLs
- [ ] Alternate language links

## 5. Internationalisation

### Structure
```rust
// i18n/en.json
{
    "common": {
        "menu": {
            "home": "Home",
            "about": "About",
            "contact": "Contact"
        }
    }
}
```

### Implementation
- [ ] Language detection
- [ ] Language switching
- [ ] URL localization
- [ ] Content translation
- [ ] RTL support
- [ ] Date/number formatting

## 6. Optimisation des Performances

### Approche de Test
```rust
#[test]
fn test_performance_metrics() {
    // Test load time
    // Test bundle size
    // Test rendering performance
}
```

### Tasks
- [ ] Code splitting
- [ ] Asset optimization
- [ ] Lazy loading
- [ ] Cache strategy
- [ ] Bundle analysis
- [ ] Performance monitoring

## 7. Stratégie de Test

### Unit Tests
- Components
- Hooks
- Utils
- Services

### Integration Tests
- Page flows
- User journeys
- API integration

### E2E Tests
- Critical paths
- User scenarios
- Cross-browser testing

## 8. Pipeline de Déploiement

### Steps
1. Build optimization
2. Asset compression
3. Environment configuration
4. CI/CD setup
5. Monitoring implementation

## Métriques de Réussite

- [ ] Lighthouse score > 90
- [ ] WCAG 2.1 AA compliance
- [ ] 100% test coverage
- [ ] < 2s initial load time
- [ ] < 100ms TTI
- [ ] Perfect accessibility score
- [ ] SEO score > 90

## Directives de Développement

1. **TDD Process**
   - Write test first
   - Implement feature
   - Refactor
   - Verify accessibility
   - Optimize performance

2. **Qualité du Code**
   - Utilisation des idiomes Rust
   - Respect des bonnes pratiques Dioxus
   - Documentation des APIs publiques
   - Maintien d'un style cohérent

3. **Accessibility First**
   - Test with screen readers
   - Ensure keyboard navigation
   - Maintain ARIA compliance
   - Regular accessibility audits

4. **SEO Best Practices**
   - Semantic HTML
   - Meta tags optimization
   - Performance optimization
   - Mobile responsiveness

5. **Performance Budget**
   - Initial bundle < 100KB
   - Page load < 2s
   - TTI < 100ms
   - FCP < 1s
