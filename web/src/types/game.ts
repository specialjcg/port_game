// TypeScript types for game state

export interface Ship {
  id: number;
  containers: number;
  containers_remaining: number;
  arrival_time: number;
  is_docked: boolean;
  docked_at?: number;
  assigned_cranes: number[];
}

export interface Berth {
  id: number;
  is_free: boolean;
  occupied_by?: number;
}

export interface Crane {
  id: number;
  is_free: boolean;
  assigned_to?: number;
  processing_speed: number;
}

export interface PortState {
  player_id: string;
  ships: Ship[];
  berths: Berth[];
  cranes: Crane[];
  score: number;
  current_time: number;
}

export interface RandomEvent {
  type: string;
  description: string;
}

export interface ActiveEffect {
  description: string;
  turns_remaining: number;
}

export interface GameState {
  playerPort: PortState;
  aiPort: PortState;
  currentTurn: number;
  isGameOver: boolean;
  winner?: string;
  activeEvents: RandomEvent[];
  activeEffects: ActiveEffect[];
}
