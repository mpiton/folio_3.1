# Cahier des charges : Portfolio de Mathieu Piton, Développeur Web, Mobile et Software

## 1. Présentation du projet

Ce projet consiste en la création d'un portfolio en ligne pour Mathieu Piton, un développeur spécialisé en web, mobile et software. Le portfolio permettra à Mathieu de se présenter, de partager ses compétences et de faciliter la prise de contact. Le projet est développé en suivant les bonnes pratiques de développement pour garantir une architecture robuste, testable et évolutive.

## 2. Objectifs

-   **Principal objectif** : Créer un portfolio performant et bien structuré qui présente les compétences de Mathieu Piton et permet une prise de contact simple.
-   **Fonctionnalités clés** :
    -   Présentation de Mathieu (bio, compétences, technologies)
    -   Formulaire de contact fonctionnel et sécurisé
    -   Logo personnalisé sur fond noir
    -   Design moderne et immersif avec des animations discrètes
    -   Flux RSS dynamique intégré

## 3. Technologies utilisées

-   **Frontend** :
    -   Astro avec TypeScript pour l'interface utilisateur
    -   TailwindCSS pour le style et la mise en page
-   **Backend** :
    -   Rust avec Axum pour l'API et la logique métier
    -   Base de données MongoDB pour le stockage des données (flux RSS, messages de contact).
-   **Testing** :
    -   Tests E2E avec Playwright pour le frontend
    -   Tests unitaires en Rust pour le backend

## 4. Pratiques de développement

### Stratégie de Test

Le projet adopte une approche pragmatique des tests pour assurer la qualité :

-   **Tests unitaires (Backend)** : La logique métier critique (services, modèles) est testée de manière isolée pour garantir son bon fonctionnement.
-   **Tests End-to-End (Frontend)** : Les parcours utilisateurs principaux (navigation, soumission du formulaire de contact) sont validés via des tests automatisés avec Playwright, simulant un utilisateur réel.
-   **CI (Intégration Continue)** : Un workflow GitHub Actions exécute automatiquement les linters et les tests à chaque push pour garantir la non-régression et la qualité du code.

### Structure du Code

Le projet est structuré pour une séparation claire des préoccupations, facilitant la maintenance et l'évolution :

-   **API Backend** : Organisée en `routes`, `services`, `models` et `middleware` pour une gestion claire de la logique.
-   **Application Frontend** : Structurée avec des composants Astro réutilisables, des layouts pour la structure des pages et des pages pour chaque route du site.

## 5. Arborescence du site

L'architecture du site est basée sur une séparation claire des préoccupations :

-   **Backend** :
    -   API REST en Rust avec Axum
    -   Interaction avec la base de données MongoDB
    -   Gestion des requêtes entrantes (formulaire de contact, récupération des flux RSS)
-   **Frontend** :
    -   Application web en Astro avec TypeScript
    -   Composants réutilisables (`.astro`)
    -   Layouts pour la structure commune
    -   Pages pour chaque route
    -   Communication avec l'API backend pour les données dynamiques

## 6. Design et expérience utilisateur

-   **Style visuel** :
    -   Moderne et épuré avec une utilisation de couleurs sobres.
    -   Animations fluides et non intrusives.
    -   Design responsive avec approche mobile-first.
    -   Optimisation des performances grâce à la génération statique d'Astro.
-   **Accessibilité** : Le design s'efforce de respecter les standards d'accessibilité (WCAG).

## 7. Gestion des tests et déploiement

-   **Tests** :
    -   Backend : Tests unitaires exécutés via `cargo test`
    -   Frontend : Tests E2E avec Playwright exécutés via `npm run test:e2e`
    -   Analyse statique : `Clippy` pour Rust et `ESLint/Prettier` pour le frontend.
-   **Déploiement** :
    -   Le déploiement est automatisé via GitHub Actions.
    -   Les environnements cibles peuvent être des plateformes comme Vercel (frontend) et un VPS (backend).

## 8. Livrables

-   Code source complet et versionné sur GitHub.
-   Documentation projet (`/dev`) décrivant l'architecture, les choix techniques et l'état du projet.
-   Version finale déployée et fonctionnelle.

## 9. Tests d'acceptation

Les tests d'acceptation permettent de vérifier que le site fonctionne comme prévu :

-   Tests utilisateur sur la prise de contact.
-   Tests de performance (Lighthouse, Web Vitals).
-   Tests de sécurité sur le formulaire de contact (validation des entrées).
-   Tests d'accessibilité (vérification manuelle et/ou automatisée).
-   Tests de responsive design sur différentes tailles d'écran.
-   Vérification du bon fonctionnement du flux RSS.
