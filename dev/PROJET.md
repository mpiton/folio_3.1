# Plan de projet : Portfolio de Mathieu Piton, Développeur Web, Mobile et Software

## 1. Objectifs

L'objectif principal de ce projet est de créer un portfolio en ligne pour Mathieu Piton, un développeur spécialisé en web, mobile et software. Le portfolio permettra à Mathieu de se présenter, de partager ses compétences et de faciliter la prise de contact. Le projet sera développé en utilisant les meilleures pratiques de Test Driven Development (TDD) et de Domain-Driven Design (DDD) pour garantir une architecture robuste, testable et évolutive.

## 2. Approche technique

Le projet sera développé avec les technologies suivantes :

- Frontend : Astro avec TypeScript pour le développement de l'interface utilisateur
- Backend : Axum pour l'API et la logique métier (migration depuis Actix Web terminée)
- Base de données : MongoDB pour le stockage des données (flux RSS, messages de contact, etc.)
- Tests : Tests E2E avec Playwright pour le frontend, tests unitaires et d'intégration avec wiremock pour les tests HTTP
- Email : Service Brevo (anciennement Sendinblue) pour l'envoi d'emails
- Cache : Cache en mémoire avec tokio pour les flux RSS

## 3. Structure du projet

Le projet est organisé selon l'arborescence suivante :

```
portfolio/
  ├── api/
  │   ├── src/
  │   │   ├── main.rs
  │   │   ├── routes/
  │   │   │   ├── health.rs
  │   │   │   ├── contact.rs
  │   │   │   └── rss.rs
  │   │   ├── models/
  │   │   │   ├── contact.rs
  │   │   │   └── rss.rs
  │   │   └── services/
  │   │       ├── contact.rs
  │   │       ├── db.rs
  │   │       ├── email_queue.rs
  │   │       ├── email_templates.rs
  │   │       └── rss.rs
  │   ├── tests/
  │   └── Cargo.toml
  ├── web/
  │   ├── src/
  │   │   ├── components/
  │   │   │   ├── common/
  │   │   │   ├── layout/
  │   │   │   └── sections/
  │   │   ├── layouts/
  │   │   │   └── Layout.astro
  │   │   ├── pages/
  │   │   │   ├── index.astro
  │   │   │   ├── about.astro
  │   │   │   ├── contact.astro
  │   │   │   └── rss.xml.js
  │   │   ├── content/
  │   │   ├── styles/
  │   │   └── utils/
  │   ├── public/
  │   ├── tests/
  │   │   └── e2e/
  │   │       └── about.spec.ts
  │   └── astro.config.mjs
  ├── db/
  │   └── mongodb/
  ├── .github/
  │   └── workflows/
  │       └── playwright.yml
  ├── .gitignore
  └── README.md
```

## 4. Fonctionnalités implémentées

### Backend (API)

1. Service RSS ✅
   - Synchronisation avec la base de données RSS
   - Mise en cache des flux RSS
   - Endpoint pour récupérer les articles récents
   - Synchronisation périodique automatique
   - Tests unitaires et d'intégration

2. Service Contact ✅
   - Validation des formulaires de contact
   - Protection anti-spam (rate limiting, détection de spam)
   - File d'attente d'emails asynchrone
   - Templates d'emails HTML
   - Intégration avec Brevo
   - Tests unitaires et d'intégration

3. Base de données ✅
   - Connexion MongoDB
   - Collections pour les flux RSS et les articles
   - Indexation pour les performances
   - Tests avec base de données de test

### Frontend (Web) 🚧

1. Pages ✅
   - [x] Layout principal
   - [x] Page À propos
   - [x] Page d'accueil
   - [x] Page Contact
   - [x] Flux RSS

2. Composants ✅
   - [x] Header avec navigation
   - [x] Footer
   - [x] Section À propos
   - [x] Formulaire de contact avec validation
   - [x] Composants communs
     - [x] Modal (small/large)
     - [x] Toast (success/error)
     - [x] Input avec validation
     - [x] Button avec variants
   - [x] Affichage des flux RSS

2. Tests Frontend ⏳
   - [x] Configuration Playwright
   - [x] Tests E2E de base
   - [x] Tests de la page À propos
   - [x] Tests de la page d'accueil
   - [x] Tests du formulaire de contact
   - [x] Tests des composants communs
     - [x] Modal tests
     - [x] Toast tests
     - [x] Input validation
   - [x] Tests des flux RSS
   - [x] Tests de navigation
   - [ ] Tests i18n

3. Performance Frontend ⏳
   - [x] Optimisation des images
   - [x] Lazy loading
   - [x] Gestion des erreurs robuste
   - [x] Tests E2E fiables
   - [ ] Code splitting
   - [ ] Bundle optimization
   - [ ] Prefetching

## 5. Tests

1. Tests Backend ✅
   - Tests unitaires ✅
   - Tests d'intégration ✅

2. Tests Frontend 🚧
   - [x] Configuration Playwright
   - [x] Tests E2E de base
   - [x] Tests de la page À propos
   - [x] Tests de la page d'accueil
   - [x] Tests du formulaire de contact
   - [x] Tests des composants communs
     - [x] Modal tests
     - [x] Toast tests
     - [x] Input validation
   - [x] Tests des flux RSS
   - [x] Tests de navigation
   - [ ] Tests i18n

## 6. Sécurité ⏳

1. Protection anti-spam ✅
   - Rate limiting par IP
   - Détection de mots-clés spam
   - Validation des champs de formulaire
   - Délai minimum entre les soumissions

2. Base de données ✅
   - Indexes uniques pour éviter les doublons
   - Validation des données

## 7. Performance ⏳

1. Cache ✅
   - Mise en cache des flux RSS
   - Actualisation périodique en arrière-plan

2. Base de données ✅
   - Indexes optimisés
   - Requêtes paginées
   - Upsert pour éviter les doublons

3. Frontend 🚧
   - [x] Optimisation des images
   - [x] Lazy loading
   - [ ] Code splitting
   - [ ] Bundle optimization
   - [ ] Prefetching

## 8. Prochaines étapes

1. Frontend
   - Compléter les pages manquantes
   - Ajouter les composants réutilisables
   - Finaliser l'internationalisation
   - Optimiser les performances
   - Compléter les tests E2E

2. Optimisations
   - Documentation API OpenAPI/Swagger
   - Monitoring temps réel
   - Système de backup automatique

3. Sécurité
   - Audit de sécurité
   - Tests de pénétration
   - Monitoring de sécurité

## 9. Déploiement

Le déploiement se fera sur :
- Backend : VPS avec Docker
- Frontend : Vercel
- Base de données : MongoDB Atlas

## 10. Maintenance

1. Logs ✅
   - Logs d'erreurs pour la synchronisation RSS
   - Logs d'envoi d'emails
   - Logs de détection de spam

2. Monitoring ✅
   - Surveillance de la synchronisation RSS
   - Surveillance de la file d'attente d'emails
   - Métriques de performance

## 11. Conventions de codage

Pour assurer la cohérence et la lisibilité du code, les conventions suivantes seront appliquées :

1. Style de code Frontend (Astro)
   - Composants en `PascalCase.astro`
   - Scripts en `camelCase.ts`
   - Styles en `kebab-case.css`
   - Variables en `camelCase`
   - Constantes en `SCREAMING_SNAKE_CASE`

2. Documentation
   - Commentaires en français
   - Documentation JSDoc pour les composants
   - Documentation TypeScript pour les types
   - Tests comme documentation vivante

3. Organisation du code Frontend
   - Un composant par fichier
   - Tests dans des fichiers séparés
   - Imports groupés et ordonnés
   - Utilisation des layouts Astro

4. Tests Frontend
   - Tests unitaires pour les composants
   - Tests d'intégration pour les pages
   - Tests d'accessibilité
   - Tests de performance

5. Gestion des erreurs Frontend
   - Gestion des erreurs côté client
   - Pages d'erreur personnalisées
   - Fallbacks pour le contenu dynamique
   - Validation des formulaires côté client

6. Performance Frontend
   - Optimisation des images avec @astrojs/image
   - Code splitting automatique
   - Prefetching intelligent
   - Optimisation du CSS

7. Sécurité Frontend
   - Protection XSS
   - CSP headers
   - Validation des entrées
   - Sécurisation des formulaires

8. Git
   - Messages de commit descriptifs en français
   - Une fonctionnalité par branche
   - Revue de code obligatoire
   - Tests passants avant merge

## 12. Étapes de développement (TDD)

1. Pour chaque fonctionnalité :
   - Écrire les tests d'acceptation
   - Écrire les tests unitaires
   - Implémenter le code minimal
   - Refactorer si nécessaire
   - Documenter le code
   - Revue de code

2. Cycle de développement :
   - Branche feature
   - Tests en rouge
   - Implémentation minimale
   - Tests en vert
   - Refactoring
   - Documentation
   - Pull request
   - Revue
   - Merge

## 13. Métriques de qualité

1. Code
   - Couverture de tests > 80%
   - Pas d'avertissements clippy
   - Documentation complète
   - Performance optimale

2. Performance
   - Temps de réponse API < 100ms
   - Utilisation mémoire stable
   - Charge CPU raisonnable
   - Temps de synchronisation RSS optimal

3. Base de données
   - Temps de requête < 50ms
   - Indexes optimisés
   - Utilisation mémoire contrôlée
   - Backup régulier

4. Sécurité
   - Pas de vulnérabilités connues
   - Protection anti-spam efficace
   - Données sensibles protégées
   - Logs sécurisés
