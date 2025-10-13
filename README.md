# 🚢 Port Terminal Manager - MCTS Strategy Game

Un jeu de stratégie où vous affrontez une IA basée sur **Monte Carlo Tree Search (MCTS)** pour gérer un terminal portuaire.

## 🏗️ Architecture

Ce projet suit une architecture moderne et robuste :

- **CQRS (Command Query Responsibility Segregation)** : Séparation des opérations de lecture et d'écriture
- **Event Sourcing** : Tous les changements d'état sont stockés comme événements immuables
- **DDD (Domain-Driven Design)** : Modélisation centrée sur le domaine métier
- **Hexagonal Architecture** : Découplage entre domaine et infrastructure
- **WebAssembly Integration** : Interface web performante via WASM

```
src/
├── domain/           # Logique métier pure (entités, événements, agrégats)
├── application/      # Cas d'usage, orchestration (CQRS)
├── infrastructure/   # Détails techniques
├── mcts/            # Intelligence Artificielle
├── game/            # Orchestration haut niveau
├── wasm.rs          # Bindings WebAssembly
└── main.rs          # CLI

web/                 # Interface utilisateur web
├── src/             # Code source React/TypeScript
└── public/          # Assets statiques
```

## ✨ Fonctionnalités

### ✅ Fonctionnalités Actuelles (v0.2.0)

- ✅ Architecture CQRS + Event Sourcing complète
- ✅ Moteur MCTS avec exploration UCB1
- ✅ Gestion de navires, quais et grues
- ✅ Event Store avec export/import JSON
- ✅ Interface Web React avec WASM
- ✅ Gestion complète du cycle de fin de tour
- ✅ Événements aléatoires (tempête, inspection, etc.)
- ✅ Système de scoring en temps réel
- ✅ Tests d'intégration WASM
- ✅ Plus de 50 tests unitaires et d'intégration

### 🚀 Prochaines Étapes

#### Phase 1 : Améliorations Interface Web
- [ ] Visualisation améliorée des événements
- [ ] Animations des mouvements de grues
- [ ] Tutorial interactif
- [ ] Mode replay des parties

#### Phase 2 : Features Avancées
- [ ] Modes de jeu multiples (Tutorial, Challenge, Sandbox)
- [ ] Système de progression / achievements
- [ ] Sauvegarde/chargement de parties
- [ ] Benchmarks MCTS (Criterion.rs)

## 🎮 Utilisation

### Développement

```bash
# Cloner le projet
git clone https://github.com/specialjcg/port_game.git
cd port_game

# Compiler le backend Rust
cargo build --release

# Construire le WASM
wasm-pack build --target web

# Démarrer l'interface web (dans le dossier web/)
cd web
npm install
npm run dev
```

### Lancer le jeu

```bash
# Mode développement
cargo run

# Mode release
cargo run --release
```

### Tests

```bash
# Tests Rust
cargo test

# Tests d'intégration WASM
wasm-pack test --firefox
```

## 🌐 Interface Web

L'interface web est accessible via :
```bash
http://localhost:5173
```

### Fonctionnalités Web Principales

- Visualisation en temps réel de l'état du port
- Drag & drop pour l'assignation des grues
- Gestion intuitive des fins de tour
- Affichage des événements aléatoires
- Calcul et affichage du score en direct

## 📚 Documentation

Pour plus de détails sur l'implémentation :
- [Architecture CQRS/ES](WEB_INSTRUCTIONS.md)
- [Interface WASM](WEB_README.md)
- [Guide du développeur](CONTRIBUTING.md)

## 🤝 Contribution

Les contributions sont bienvenues ! Consultez [CONTRIBUTING.md](CONTRIBUTING.md) pour les détails.

---

**Développé avec ❤️ en Rust** 🦀

Suivant les meilleures pratiques 2025 :
- Clean Architecture
- CQRS + Event Sourcing
- TDD
- DDD
