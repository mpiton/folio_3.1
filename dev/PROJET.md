# Plan de projet : Portfolio de Mathieu Piton, Développeur Web, Mobile et Software

## 1. Objectifs

L'objectif principal de ce projet est de créer un portfolio en ligne pour Mathieu Piton, un développeur spécialisé en web, mobile et software. Le portfolio permettra à Mathieu de se présenter, de partager ses compétences et de faciliter la prise de contact.

## 2. Approche technique

Le projet est développé avec les technologies suivantes :

-   **Frontend** : Astro avec TypeScript pour l'interface utilisateur.
-   **Backend** : Axum (Rust) pour l'API et la logique métier.
-   **Base de données** : MongoDB pour le stockage des données (flux RSS, messages de contact).
-   **Tests** : Tests E2E avec Playwright pour le frontend et tests unitaires pour le backend.
-   **Email** : Service Brevo pour l'envoi d'emails transactionnels.
-   **Cache** : Cache en mémoire avec Tokio pour les flux RSS.

## 3. Structure du projet

Le projet est organisé selon l'arborescence suivante :

```
portfolio/
  ├── api/
  │   ├── src/
  │   │   ├── bin/
  │   │   │   ├── backup_db.rs
  │   │   │   ├── clean_tweets.rs
  │   │   │   └── sync_rss.rs
  │   │   ├── main.rs
  │   │   ├── lib.rs
  │   │   ├── config.rs
  │   │   ├── routes/
  │   │   ├── models/
  │   │   └── services/
  │   ├── Cargo.toml
  │   └── docker-compose.yml
  ├── web/
  │   ├── src/
  │   │   ├── components/
  │   │   │   ├── common/
  │   │   │   ├── layout/
  │   │   │   └── sections/
  │   │   ├── layouts/
  │   │   ├── pages/
  │   │   ├── assets/
  │   │   └── scripts/
  │   ├── public/
  │   ├── e2e/
  │   ├── astro.config.mjs
  │   └── package.json
  ├── .github/
  │   └── workflows/
  │       └── ci.yml
  └── .gitignore
```

## 4. Fonctionnalités implémentées

### Backend (API)

1.  **Service RSS** ✅
    -   Synchronisation avec la base de données RSS.
    -   Mise en cache des flux RSS.
    -   Endpoint pour récupérer les articles récents.
    -   Script de synchronisation manuel (`sync_rss.rs`).
    -   Tests unitaires.

2.  **Service Contact** ✅
    -   Validation des formulaires de contact.
    -   Protection anti-spam (rate limiting par IP).
    -   File d'attente d'emails asynchrone.
    -   Templates d'emails HTML.
    -   Intégration avec Brevo.
    -   Tests unitaires.

3.  **Base de données** ✅
    -   Connexion à MongoDB.
    -   Collections pour les flux RSS et les contacts.
    -   Indexation pour les performances (y compris TTL).
    -   Script de sauvegarde manuel (`backup_db.rs`).

### Frontend (Web) 🚧

1.  **Pages** ✅
    -   Layout principal.
    -   Page d'accueil, À propos, Contact, Mentions Légales.
    -   Page d'affichage des flux RSS.
    -   Page de démonstration des composants (utilisée pour les tests E2E).

2.  **Composants** ✅
    -   Header avec navigation et Footer.
    -   Formulaire de contact avec validation côté client.
    -   Affichage des flux RSS.
    -   Composants communs (Button, Input, Card, Toast) utilisés dans l'application.

3.  **Tests Frontend** ✅
    -   Configuration Playwright pour les tests E2E.
    -   Tests E2E sur les pages principales et les composants via la page de démo.

4.  **Performance Frontend** ⏳
    -   Optimisation des images (`astro:assets`).
    -   PWA basique configurée.
    -   Lazy loading des images.

## 5. Tests

1.  **Tests Backend** ✅
    -   Tests unitaires sur les modèles et services.
    -   *Note : Les tests d'intégration ont été supprimés car ils nécessitaient un environnement de base de données complexe et instable.*

2.  **Tests Frontend** 🚧
    -   Tests E2E avec Playwright.
    -   *Note : Pas de tests unitaires ou de composants (type Vitest/Testing Library) actuellement.*

## 6. Sécurité

1.  **Protection anti-spam** ✅
    -   Rate limiting par IP sur l'API de contact.
    -   Validation rigoureuse des champs du formulaire côté backend.

2.  **Base de données** ✅
    -   Indexes uniques pour éviter les doublons.
    -   Sanitisation des entrées pour prévenir les injections NoSQL.

## 7. Performance

1.  **Cache** ✅
    -   Mise en cache des flux RSS en mémoire pour réduire les appels à la base de données.

2.  **Base de données** ✅
    -   Indexes optimisés pour des requêtes rapides.

3.  **Frontend** 🚧
    -   Optimisation des images.
    -   Build statique avec Astro pour des temps de chargement rapides.

## 8. Prochaines étapes et améliorations futures

1.  **Fonctionnalités**
    -   Ajouter un support multilingue (i18n).
    -   Mettre en place un système de recherche sur les articles RSS.

2.  **Optimisations**
    -   Générer une documentation API (ex: OpenAPI/Swagger).
    -   Mettre en place un monitoring plus avancé (ex: Prometheus/Grafana).
    -   Automatiser le script de backup via un cronjob ou un service systemd.

3.  **Tests**
    -   Réintroduire des tests d'intégration backend avec un environnement de test stable.
    -   Mettre en place des tests unitaires pour les composants frontend critiques.
