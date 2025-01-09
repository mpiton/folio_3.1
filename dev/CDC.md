# Cahier des charges : Portfolio de Mathieu Piton, Développeur Web, Mobile et Software

## 1. Présentation du projet

Ce projet consiste en la création d'un portfolio en ligne pour Mathieu Piton, un développeur spécialisé en web, mobile et software. Le portfolio permettra à Mathieu de se présenter, de partager ses compétences et de faciliter la prise de contact. Le projet sera développé en utilisant les meilleures pratiques de Test Driven Development (TDD) et de Domain-Driven Design (DDD) pour garantir une architecture robuste, testable et évolutive.

## 2. Objectifs

    Principal objectif : Créer un portfolio performant, bien structuré et testable qui présente les compétences de Mathieu Piton et permet une prise de contact simple.
    Fonctionnalités clés :
        Présentation de Mathieu (bio, compétences, technologies)
        Formulaire de contact fonctionnel
        Logo personnalisé sur fond noir
        Design moderne et immersif
        Développement selon les principes TDD et DDD
        Support multilingue (français et anglais) pour toutes les pages et fonctionnalités

## 3. Technologies utilisées

    Frontend :
        Rust avec le framework Yew ou Percy pour le développement de l'interface utilisateur
        Three.js pour des éléments interactifs en 3D
        HTML, CSS (préprocesseur comme Sass ou TailwindCSS) pour le style et la mise en page
    Backend :
        Rust avec un framework web comme Rocket, Actix Web ou Warp pour l'API et la logique métier
        Base de données MongoDB pour le stockage des données (flux RSS, messages de contact, etc.)
    Testing : Tests unitaires et d'intégration en Rust avec les outils suivants :
        Tests unitaires via le module de test intégré de Rust
        Tests d'intégration avec des bibliothèques comme reqwest ou httpmock pour tester l'API
    Logo : Intégration du logo existant de Mathieu.

## 4. Pratiques de développement

### Test Driven Development (TDD)

Le développement du portfolio respectera les principes de TDD, à savoir :

    Écrire des tests avant le code de production.
    Tests unitaires : Chaque fonction ou composant sera testé indépendamment dès sa conception.
    Tests d'intégration : Vérification de l'interaction correcte entre les différentes parties du système (API backend, composants frontend, base de données, etc.).
    Tests fonctionnels : Tests des flux utilisateurs principaux pour vérifier la fonctionnalité du site dans son ensemble.
    Exemples :
        Tester les routes de l'API backend avec des requêtes HTTP simulées
        Tester le rendu et les interactions des composants Yew/Percy
        Tester l'intégration avec la base de données MongoDB
        Tester les animations Three.js

### Domain-Driven Design (DDD)

Le projet respectera les principes du DDD pour organiser les données et la logique métier autour du domaine du développement web de Mathieu :

    Définition des agrégats : Le projet sera divisé en domaines bien distincts. Par exemple, un agrégat pourrait être l'entité "Utilisateur" (Mathieu Piton, avec ses compétences, son expérience, etc.), un autre pourrait être "Contact" (formulaire de contact, validation des champs, etc.).
    Modélisation du domaine : Les objets métiers seront bien définis, avec des règles de validation (ex : validation du formulaire de contact).
    Isolation des dépendances : Le code sera structuré de manière à ce que les différentes parties du système soient découplées et indépendantes les unes des autres. Par exemple, la logique du formulaire de contact et celle de la présentation des compétences seront isolées.
    Langage Ubiquitaire : Le code sera rédigé dans un langage compréhensible pour toutes les parties prenantes du projet, avec des noms de classes et de fonctions qui décrivent clairement le domaine.

## 5. Arborescence du site

L'architecture du site sera basée sur une approche DDD, avec une séparation claire des préoccupations :

    Backend :
        API REST en Rust avec Rocket, Actix Web ou Warp
        Interaction avec la base de données MongoDB
        Gestion des requêtes entrantes (formulaire de contact, récupération des flux RSS, etc.)
    Frontend :
        Application web en Rust avec Yew ou Percy
        Composants réutilisables pour les différentes sections (header, footer, about, contact, rss)
        Communication avec l'API backend pour les données dynamiques
        Intégration des animations Three.js
    Tests :
        Tests unitaires et d'intégration pour chaque partie du système (API, composants, base de données)
        Tests fonctionnels pour vérifier le bon fonctionnement global du site

L'architecture suivra le modèle Model-View-Controller (MVC) ou un modèle similaire, mais ajusté aux besoins de l'application. Les données seront structurées de manière à ce que le système puisse évoluer facilement à l'avenir.

## 6. Design et expérience utilisateur

    Style visuel :
        Moderne et épuré avec une utilisation de couleurs sobres et élégantes, comme du noir, blanc et une couleur principale.
        Animation immersive via Three.js pour un environnement interactif et captivant.
        Responsivité : Le site doit être parfaitement fonctionnel sur toutes les tailles d'écrans, particulièrement sur les mobiles et tablettes.
    Tests d'accessibilité : Le design devra respecter les standards WCAG pour l'accessibilité.

## 7. Gestion des tests et déploiement

    Tests : Tous les tests (unitaires, intégration, fonctionnels) seront écrits en Rust et exécutés via Cargo. Ils seront lancés automatiquement à chaque changement de code via l'intégration continue.
    Déploiement :
        Le code sera déployé via un service d'intégration continue comme GitHub Actions, où les tests seront exécutés automatiquement avant chaque déploiement.
        Le site sera hébergé sur Vercel, avec une configuration spécifique pour les applications Rust.
        La connexion à la base de données MongoDB sera configurée via des variables d'environnement sécurisées.

## 8. Livrables

    Code source complet et versionné dans un dépôt Git.
    Documentation détaillée sur le projet, les choix d'architecture, les tests et les étapes de déploiement.
    Version finale déployée, avec un accès pour tester la version en ligne avant validation finale.

## 9. Tests d'acceptation

Les tests d'acceptation permettront de vérifier que le site fonctionne comme prévu :

    Tests utilisateur sur la prise de contact, dans les deux langues (français et anglais)
    Tests de performance pour vérifier le temps de chargement du site, notamment les animations en Three.js
    Tests de sécurité pour le formulaire de contact (protection contre le spam, validation des entrées)
    Vérification du bon fonctionnement du changement de langue et de l'affichage correct des traductions
