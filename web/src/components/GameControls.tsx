import React from 'react';
import type { GameState } from '../types/game';

interface GameControlsProps {
  gameState: GameState;
  onEndTurn: () => void;
  onSpawnShips: (count: number) => void;
}

export const GameControls: React.FC<GameControlsProps> = ({
  gameState,
  onEndTurn,
  onSpawnShips
}) => {
  return (
    <div className="game-controls">
      <div className="controls-header">
        <h2>🎮 Game Controls</h2>
        <div className="turn-counter">
          Turn: <span className="turn-number">{gameState.currentTurn}</span>
        </div>
      </div>

      <div className="controls-actions">
        <button
          className="btn btn-primary btn-large"
          onClick={onEndTurn}
          disabled={gameState.isGameOver}
        >
          ⏭️ End Turn
        </button>

        <button
          className="btn btn-secondary"
          onClick={() => onSpawnShips(1)}
          disabled={gameState.isGameOver}
        >
          ➕ Spawn 1 Ship
        </button>

        <button
          className="btn btn-secondary"
          onClick={() => onSpawnShips(3)}
          disabled={gameState.isGameOver}
        >
          ➕ Spawn 3 Ships
        </button>
      </div>

      {/* Active Effects */}
      {gameState.activeEffects && gameState.activeEffects.length > 0 && (
        <div className="active-effects">
          <h3>⚡ Active Effects</h3>
          <div className="effects-list">
            {gameState.activeEffects.map((effect, idx) => (
              <div key={idx} className="effect-item">
                <span className="effect-description">{effect.description}</span>
                <span className="effect-turns">
                  ({effect.turns_remaining} turns)
                </span>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Recent Events */}
      {gameState.activeEvents && gameState.activeEvents.length > 0 && (
        <div className="recent-events">
          <h3>📰 Recent Events</h3>
          <div className="events-list">
            {gameState.activeEvents.map((event, idx) => (
              <div key={idx} className="event-item">
                {event.description}
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Game Over */}
      {gameState.isGameOver && (
        <div className="game-over">
          <h2>🎉 Game Over!</h2>
          <div className="winner">
            {gameState.winner === 'player' && '🏆 You Win!'}
            {gameState.winner === 'ai' && '🤖 AI Wins!'}
            {gameState.winner === 'tie' && '🤝 Tie Game!'}
          </div>
          <div className="final-scores">
            <div>Player: {gameState.playerPort.score}</div>
            <div>AI: {gameState.aiPort.score}</div>
          </div>
        </div>
      )}
    </div>
  );
};
