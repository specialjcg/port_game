# 🚢 Port Terminal Manager - MCTS Strategy Game

Un jeu de stratégie où vous affrontez une IA basée sur **Monte Carlo Tree Search (MCTS)** pour gérer un terminal portuaire.

## 🏗️ Architecture

Ce projet suit une architecture moderne et robuste :

- **CQRS (Command Query Responsibility Segregation)** : Séparation des opérations de lecture et d'écriture
- **Event Sourcing** : Tous les changements d'état sont stockés comme événements immuables
- **DDD (Domain-Driven Design)** : Modélisation centrée sur le domaine métier
- **Hexagonal Architecture** : Découplage entre domaine et infrastructure

```
src/
├── domain/           # Logique métier pure (entités, événements, agrégats)
│   ├── entities.rs   # Ship, Berth, Crane
│   ├── events.rs     # DomainEvent (Event Sourcing)
│   ├── value_objects.rs # Identifiants typés
│   └── aggregates.rs # Port (aggregate root)
│
├── application/      # Cas d'usage, orchestration (CQRS)
│   ├── commands.rs   # Commandes (write)
│   ├── queries.rs    # Requêtes (read)
│   └── handlers.rs   # Command/Query handlers
│
├── infrastructure/   # Détails techniques
│   └── event_store.rs # InMemoryEventStore (peut être remplacé par DB)
│
├── mcts/             # Intelligence Artificielle
│   ├── mod.rs        # Moteur MCTS
│   ├── tree.rs       # Arbre de recherche UCB1
│   ├── actions.rs    # Actions possibles
│   └── simulation.rs # Simulations
│
├── game/             # Orchestration haut niveau
│   └── mod.rs        # GameSession, GameMode
│
└── main.rs           # CLI
```

## ✨ Fonctionnalités

### ✅ MVP Actuel (v0.1.0)

- ✅ Architecture CQRS + Event Sourcing complète
- ✅ Moteur MCTS avec exploration UCB1
- ✅ Gestion de navires, quais et grues
- ✅ Event Store avec export/import JSON (replay de parties)
- ✅ 31 tests unitaires (100% de passage)
- ✅ CLI basique pour visualiser l'état du jeu

### 🚀 Prochaines Étapes

#### Phase 1 : Gameplay Interactif
- [ ] CLI interactive pour les actions du joueur
  - Amarrer un navire (DockShip)
  - Assigner une grue (AssignCrane)
  - Terminer son tour (EndTurn)
- [ ] Tour par tour : Joueur → IA MCTS
- [ ] Affichage comparatif des scores
- [ ] Visualisation de l'arbre MCTS (transparence IA)

#### Phase 2 : Simulation Complète
- [ ] Progression du temps (tours)
- [ ] Traitement des conteneurs par les grues
- [ ] Désamarrage automatique des navires terminés
- [ ] Calcul de score avancé (temps d'attente, efficacité)
- [ ] Événements aléatoires (tempête, panne, inspection)

#### Phase 3 : Features Avancées
- [ ] Modes de jeu multiples (Tutorial, Challenge, Sandbox)
- [ ] Système de progression / achievements
- [ ] Sauvegarde/chargement de parties
- [ ] Benchmarks MCTS (Criterion.rs)
- [ ] Interface Web (React + WebAssembly)

## 🎮 Utilisation

### Installation

```bash
# Cloner le projet
git clone https://github.com/specialjcg/port_game.git
cd port_game

# Compiler
cargo build --release
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
# Tous les tests
cargo test

# Tests spécifiques
cargo test domain::
cargo test mcts::

# Avec verbosité
cargo test -- --nocapture
```

### Event Sourcing - Replay

Le jeu exporte automatiquement tous les événements en JSON :

```rust
let session = GameSession::new(GameMode::VersusAI, player_id, ai_id);
// ... jouer ...
let replay_json = session.export_replay()?;
// Sauvegarder dans un fichier ou analyser
```

**Avantages** :
- Rejouer une partie exactement
- Analyser les décisions de l'IA
- Débugger facilement
- Comparer différentes stratégies MCTS

## 🧪 Tests & Qualité

### Coverage Actuel

```bash
cargo test
# 31 tests passent
```

### Standards Suivis

- **TDD (Test-Driven Development)** : Tests écrits en premier
- **SOLID** : Single Responsibility, Open/Closed, etc.
- **DRY** : Don't Repeat Yourself
- **Borrow Checker friendly** : Architecture pensée pour Rust

### Métriques

```bash
# Vérifier la compilation
cargo check

# Formater le code
cargo fmt

# Linter
cargo clippy
```

## 🧠 MCTS (Monte Carlo Tree Search)

### Configuration

```rust
MCTSConfig {
    num_simulations: 1000,     // Nombre de simulations
    exploration_constant: 1.41, // √2 (UCB1 standard)
    max_depth: 50,              // Profondeur max de l'arbre
}
```

### Algorithme

1. **Selection** : Parcours de l'arbre avec UCB1
2. **Expansion** : Ajout de nœuds enfants (actions possibles)
3. **Simulation** : Jouer aléatoirement jusqu'à la fin
4. **Backpropagation** : Mise à jour des statistiques

### UCB1 Formula

```
UCB1 = exploitation + exploration
     = (score_moyen) + C * √(ln(N_parent) / N_node)
```

## 📊 Event Sourcing - Exemples d'événements

```rust
// Navire arrive
DomainEvent::ShipArrived {
    ship_id: ShipId(1),
    container_count: 50,
    arrival_time: 0.0,
}

// Navire amarré
DomainEvent::ShipDocked {
    ship_id: ShipId(1),
    berth_id: BerthId(0),
    player: player_id,
    docking_time: 1.0,
}

// Grue assignée
DomainEvent::CraneAssigned {
    crane_id: CraneId(0),
    ship_id: ShipId(1),
    player: player_id,
    assignment_time: 2.0,
}
```

## 🛠️ Technologies

- **Rust 2021** : Langage système performant et sûr
- **serde/serde_json** : Sérialisation (Event Store)
- **uuid** : Identifiants uniques
- **chrono** : Timestamps
- **rand** : Aléatoire (MCTS simulations)
- **criterion** : Benchmarking (à venir)

## 📝 Licence

MIT

## 🤝 Contribution

Les contributions sont bienvenues ! Voir le fichier CONTRIBUTING.md (à créer).

---

**Développé avec ❤️ en Rust** 🦀

Suivant les meilleures pratiques 2025 :
- Clean Architecture
- CQRS + Event Sourcing
- TDD
- DDD
