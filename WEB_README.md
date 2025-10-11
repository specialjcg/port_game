# 🌐 Port Game - Web Version (Phase 3)

Interface Web moderne avec React + WebAssembly pour jouer dans le navigateur.

## 🏗️ Architecture Web

```
port_game/
├── src/
│   └── wasm.rs          # Rust → WASM bindings
├── web/                  # React frontend
│   ├── src/
│   │   ├── components/   # React components
│   │   ├── hooks/        # Custom React hooks
│   │   ├── wasm/         # WASM loader
│   │   └── App.tsx       # Main app
│   └── package.json
└── Cargo.toml           # WASM features
```

## 🚀 Build Instructions

### Prérequis

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install Node.js (v18+)
# https://nodejs.org/

# Verify installations
wasm-pack --version
node --version
npm --version
```

### Build WASM

```bash
# Build Rust → WebAssembly
wasm-pack build --target web --features wasm

# Or for production (optimized)
wasm-pack build --release --target web --features wasm

# Output: pkg/ directory with .wasm and .js files
```

### Setup React App

```bash
# Create React app (first time only)
cd web
npm create vite@latest . -- --template react-ts

# Install dependencies
npm install

# Install WASM loader
npm install vite-plugin-wasm vite-plugin-top-level-await

# Link Rust WASM package
npm link ../pkg
```

### Run Development Server

```bash
# Terminal 1: Watch Rust changes
cargo watch -i web/ -s "wasm-pack build --target web --features wasm"

# Terminal 2: Run React dev server
cd web
npm run dev

# Open http://localhost:5173
```

## 🎮 WASM API

### JavaScript/TypeScript Usage

```typescript
import init, { WasmGame } from 'port-game';

// Initialize WASM
await init();

// Create game instance
const game = new WasmGame();

// Game actions
game.spawnShips(3);
game.dockShip(shipId, berthId);
game.assignCrane(craneId, shipId);
game.processContainers();
game.aiTakeTurn();

// Get game state
const playerPort = game.getPlayerPort();
const aiPort = game.getAiPort();
const events = game.processRandomEvents();
const activeEffects = game.getActiveEffects();

// Game state
const turn = game.getCurrentTurn();
const isOver = game.isGameOver();
const winner = game.getWinner();

// Export replay
const replay = game.exportReplay();
```

## 📦 React Components (To Implement)

### Component Structure

```tsx
<App>
  <GameBoard>
    <PlayerPort>
      <BerthSlot />
      <ShipQueue />
      <CranePanel />
    </PlayerPort>

    <AIPort>
      <BerthSlot />
      <ShipQueue />
      <MCTSVisualization />
    </AIPort>
  </GameBoard>

  <GameControls>
    <ActionButtons />
    <TurnCounter />
    <ScoreBoard />
  </GameControls>

  <EventNotifications />
</App>
```

### Key Features

- **Drag & Drop**: Ships → Berths, Cranes → Ships
- **Real-time Updates**: React state synced with WASM
- **Animations**: CSS transitions for ship movements
- **MCTS Tree**: Interactive D3.js visualization
- **Responsive**: Mobile & desktop support

## 🎨 Styling

```bash
# Install Tailwind CSS
cd web
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p

# Or use Material-UI
npm install @mui/material @emotion/react @emotion/styled
```

## 📊 Performance

### WASM Bundle Size

```bash
# Check bundle size
ls -lh pkg/*.wasm

# Optimize further
wasm-opt -Os -o optimized.wasm pkg/port_game_bg.wasm
```

### React Performance

- Use `React.memo` for ship/berth components
- Virtualize long lists with `react-window`
- Lazy load MCTS visualization

## 🐛 Debugging

### Rust WASM

```rust
// In wasm.rs
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

log(&format!("Debug: {:?}", value));
```

### Browser Console

```javascript
// Check WASM memory
console.log(wasmMemory.buffer.byteLength);

// Profile WASM calls
console.time('aiTakeTurn');
game.aiTakeTurn();
console.timeEnd('aiTakeTurn');
```

## 🚢 Deployment

### Static Hosting (Vercel, Netlify, GitHub Pages)

```bash
# Build for production
cd web
npm run build

# Deploy dist/ folder
# Vercel: vercel deploy
# Netlify: netlify deploy --prod
# GitHub Pages: gh-pages -d dist
```

### Docker

```dockerfile
FROM rust:latest as wasm-builder
WORKDIR /app
RUN cargo install wasm-pack
COPY . .
RUN wasm-pack build --release --target web --features wasm

FROM node:18 as web-builder
WORKDIR /app/web
COPY web/package*.json ./
RUN npm ci
COPY web/ ./
COPY --from=wasm-builder /app/pkg ../pkg
RUN npm run build

FROM nginx:alpine
COPY --from=web-builder /app/web/dist /usr/share/nginx/html
EXPOSE 80
```

## 🎯 Roadmap

### Phase 3.1 - MVP Web (Current)
- [x] WASM bindings
- [ ] Basic React UI
- [ ] Drag & drop
- [ ] Deploy to Vercel

### Phase 3.2 - Enhanced UX
- [ ] Animations & transitions
- [ ] Sound effects
- [ ] Mobile responsive
- [ ] PWA support

### Phase 3.3 - Advanced Features
- [ ] MCTS tree visualization
- [ ] Multiplayer (WebRTC)
- [ ] Leaderboard (Firebase)
- [ ] Replay viewer

## 📚 Resources

- [wasm-bindgen book](https://rustwasm.github.io/wasm-bindgen/)
- [Rust + React tutorial](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm)
- [React DnD](https://react-dnd.github.io/react-dnd/)
- [D3.js](https://d3js.org/)

---

**Status**: Phase 3 WASM bindings complete, React UI in progress
