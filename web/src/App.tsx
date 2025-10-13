import React from 'react';
import { useGame } from './hooks/useGame';
import { Port } from './components/Port';
import { GameControls } from './components/GameControls';
import './App.css';

function App() {
  const { gameState, loading, error, actions } = useGame();

  if (loading) {
    return (
      <div className="loading-screen">
        <div className="spinner"></div>
        <h2>Loading Port Game...</h2>
        <p>Initializing WebAssembly...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="error-screen">
        <h2>❌ Error</h2>
        <p>{error}</p>
        <button onClick={() => window.location.reload()}>
          🔄 Reload
        </button>
      </div>
    );
  }

  if (!gameState) {
    return <div>No game state</div>;
  }

  return (
    <div className="app">
      <header className="app-header">
        <h1>🚢 Port Game - MCTS Strategy</h1>
        <p className="subtitle">Manage your port efficiently and beat the AI!</p>
      </header>

      <main className="game-layout">
        {/* Player Port */}
        <section className="player-section">
          <Port
            port={gameState.playerPort}
            title="👤 Your Port"
            isPlayer={true}
            onDockShip={actions.dockShip}
            onAssignCrane={actions.assignCrane}
          />
        </section>

        {/* Game Controls */}
        <aside className="controls-section">
          <GameControls
            gameState={gameState}
            onEndTurn={actions.endTurn}
            onSpawnShips={actions.spawnShips}
          />
        </aside>

        {/* AI Port */}
        <section className="ai-section">
          <Port
            port={gameState.aiPort}
            title="🤖 AI Port (MCTS)"
            isPlayer={false}
          />
        </section>
      </main>

      <footer className="app-footer">
        <p>
          Drag ships to berths 🚢 → ⚓ | Click cranes 🏗️ then docked ships to assign
        </p>
      </footer>
    </div>
  );
}

export default App;
