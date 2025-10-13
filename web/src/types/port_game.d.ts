declare module '@pkg/port_game' {
    export class WasmGame {
        constructor();
        startTurn(): void;
        spawnShips(count: number): void;
        dockShip(shipId: number, berthId: number): Promise<void>;
        assignCrane(craneId: number, shipId: number): Promise<void>;
        processContainers(): void;
        aiTakeTurn(): void;
        processRandomEvents(): string[];
        getPlayerPort(): any;
        getAiPort(): any;
        getCurrentTurn(): number;
        isGameOver(): boolean;
        getWinner(): string | null;
        exportReplay(): Promise<string>;
        getActiveEffects(): any[];
        getCraneEfficiency(): number;
    }

    export default function init(module: string): Promise<void>;
}
