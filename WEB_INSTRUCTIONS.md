# 🚢 Port Game - Instructions Web

## 🎮 Comment Jouer

### Démarrage Rapide

1. **Construire le package WASM** (une seule fois) :
```bash
wasm-pack build --target web --features wasm
```

2. **Lancer le serveur de développement** :
```bash
cd web
npm install  # Première fois seulement
npm run dev
```

3. **Ouvrir le navigateur** :
   - Aller sur http://localhost:5173/
   - Le jeu se charge automatiquement !

---

## 🎯 Règles du Jeu

### Objectif
Décharger plus de conteneurs que l'IA (qui utilise MCTS) pour gagner !

### Comment Jouer

#### 1. **Amarrer un Navire** 🚢 → ⚓
   - **Drag & Drop** : Glisse un navire de la zone "Waiting Ships" vers un quai libre (Berth)
   - Ou clique sur un navire puis sur un quai
   - Le navire s'amarre au quai

#### 2. **Assigner une Grue** 🏗️
   - **Clic sur grue** : Clique sur une grue libre (vert)
   - La grue est sélectionnée (indication en bas)
   - **Clic sur navire amarré** : Clique sur un navire amarré
   - La grue commence à décharger les conteneurs !

#### 3. **Fin de Tour** ⏭️
   - Clique sur "End Turn" quand tu as fini tes actions
   - Les grues assignées déchargent des conteneurs (10 par grue/tour)
   - L'IA MCTS joue son tour automatiquement
   - De nouveaux navires arrivent

#### 4. **Spawn Manuel** (optionnel)
   - "Spawn 1 Ship" : Ajoute 1 navire
   - "Spawn 3 Ships" : Ajoute 3 navires
   - Utile pour tester ou accélérer la partie

---

## 📊 Interface

### Zone Joueur (Gauche)
- **⏳ Waiting Ships** : Navires en attente d'amarrage
- **⚓ Berths** : Quais (zones de dépôt)
- **🚢 Docked Ships** : Navires amarrés avec progression
- **🏗️ Cranes** : Grues disponibles/occupées

### Contrôles (Centre)
- **Turn** : Numéro du tour actuel
- **End Turn** : Finir ton tour
- **Spawn Ships** : Ajouter des navires
- **⚡ Active Effects** : Événements en cours (tempêtes, bonus, etc.)
- **📰 Recent Events** : Derniers événements survenus

### Zone IA (Droite)
- Port de l'IA MCTS
- Observe les décisions de l'IA !
- Compare les scores

---

## 🎲 Événements Aléatoires

Des événements peuvent survenir pendant la partie :

- **🌪️ STORM** : Réduit l'efficacité des grues (-50%)
- **☀️ GOOD WEATHER** : Augmente l'efficacité (+30%)
- **🏗️ CRANE BREAKDOWN** : Une grue tombe en panne
- **📦 RUSH HOUR** : Arrivée massive de navires
- **🚨 CUSTOMS INSPECTION** : Retard de traitement

---

## 🏆 Système de Score

### Points Positifs
- **+10 points** par conteneur déchargé
- Bonus pour rapidité de traitement

### Pénalités
- **-5 points/tour** par navire en attente
- Temps d'attente = perte de points

### Fin de Partie
- La partie se termine quand tous les navires sont traités
- Le joueur avec le plus de points gagne !
- 🏆 Victoire Joueur | 🤖 Victoire IA | 🤝 Égalité

---

## 🔧 Développement

### Rebuild WASM après modifications Rust
```bash
wasm-pack build --target web --features wasm
```

### Build production
```bash
cd web
npm run build
# Output dans web/dist/
```

### Tests
```bash
# Tests Rust
cargo test

# Tests WASM
wasm-pack test --headless --firefox
```

---

## 🚀 Déploiement

### Vercel
```bash
cd web
npm run build
vercel deploy
```

### Netlify
```bash
cd web
npm run build
netlify deploy --prod --dir=dist
```

### Docker
```dockerfile
# Voir Dockerfile dans le repo
docker build -t port-game .
docker run -p 8080:80 port-game
```

---

## 💡 Conseils Stratégiques

1. **Amarrez rapidement** : Les navires en attente coûtent des points
2. **Utilisez toutes les grues** : Plus de grues = plus rapide
3. **Priorisez les gros navires** : Ils rapportent plus de points
4. **Anticipez les événements** : Les tempêtes ralentissent tout
5. **Observez l'IA** : L'algorithme MCTS est fort, apprenez de lui !

---

## 🐛 Problèmes Courants

### Le jeu ne charge pas
- Vérifiez que WASM est compilé : `ls ../pkg/port_game_bg.wasm`
- Rebuild si nécessaire : `wasm-pack build --target web --features wasm`

### Erreur au drag & drop
- Assurez-vous que le quai est libre (pas de navire amarré)
- Rechargez la page (F5)

### L'IA ne joue pas
- Attendez quelques secondes (MCTS calcule)
- Vérifiez la console navigateur (F12)

---

## 📚 Architecture Technique

- **Frontend** : React 19 + TypeScript + Vite 5.4
- **Backend** : Rust + WebAssembly (wasm-bindgen)
- **IA** : Monte Carlo Tree Search (MCTS) avec UCB1
- **Architecture** : CQRS + Event Sourcing + DDD
- **Patterns** : Hexagonal Architecture, Ports & Adapters

---

## 🎉 Bon Jeu !

**Astuce** : Utilise les raccourcis clavier (à venir) pour jouer plus vite !

---

*Généré avec ❤️ par Rust 🦀 + React ⚛️*
