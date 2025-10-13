import { useState, useEffect, useCallback } from 'react';
import type { GameState, PortState } from '../types/game';
import init, { WasmGame } from 'port_game';

export function useGame() {
  const [game, setGame] = useState<WasmGame | null>(null);
  const [gameState, setGameState] = useState<GameState | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Initialize WASM and game
  useEffect(() => {
    const initGame = async () => {
      try {
        // Initialiser le module WASM
        await init();
        // Créer une nouvelle instance du jeu
        const newGame = new WasmGame();
        newGame.spawnShips(3);
        setGame(newGame);
        updateGameState(newGame);
        setLoading(false);
      } catch (err) {
        setError(`Failed to initialize game: ${err}`);
        setLoading(false);
        console.error("Game initialization error:", err);
      }
    };

    initGame();
  }, []);

  // Update game state from WASM
  const updateGameState = useCallback((g: any) => {
    try {
      const playerPort = g.getPlayerPort() as PortState;
      const aiPort = g.getAiPort() as PortState;
      const currentTurn = g.getCurrentTurn();
      const isGameOver = g.isGameOver();
      const winner = g.getWinner();
      const activeEvents = g.processRandomEvents() as any[];
      const activeEffects = g.getActiveEffects() as any[];

      setGameState({
        playerPort,
        aiPort,
        currentTurn,
        isGameOver,
        winner: winner || undefined,
        activeEvents,
        activeEffects
      });
    } catch (err) {
      setError(`Failed to update game state: ${err}`);
    }
  }, []);

  // Game actions
  const dockShip = useCallback((shipId: number, berthId: number) => {
    if (!game) return;
    try {
      game.dockShip(shipId, berthId);
      updateGameState(game);
    } catch (err) {
      setError(`Failed to dock ship: ${err}`);
      throw err;
    }
  }, [game, updateGameState]);

  const assignCrane = useCallback((craneId: number, shipId: number) => {
    if (!game) return;
    try {
      game.assignCrane(craneId, shipId);
      updateGameState(game);
    } catch (err) {
      setError(`Failed to assign crane: ${err}`);
      throw err;
    }
  }, [game, updateGameState]);

  const endTurn = useCallback(() => {
    if (!game) return;
    try {
      // Utiliser la fonction endTurn du backend qui gère déjà la séquence complète
      game.endTurn();

      // Mettre à jour l'état
      updateGameState(game);
    } catch (err) {
      setError(`Failed to end turn: ${err}`);
      console.error("End turn error details:", err);
    }
  }, [game, updateGameState]);

  const spawnShips = useCallback((count: number) => {
    if (!game) return;
    try {
      game.spawnShips(count);
      updateGameState(game);
    } catch (err) {
      setError(`Failed to spawn ships: ${err}`);
    }
  }, [game, updateGameState]);

  return {
    gameState,
    loading,
    error,
    actions: {
      dockShip,
      assignCrane,
      endTurn,
      spawnShips
    }
  };
}
