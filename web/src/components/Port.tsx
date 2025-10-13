import React from 'react';
import type { PortState } from '../types/game';
import { Ship } from './Ship';
import { Berth } from './Berth';
import { Crane } from './Crane';

interface PortProps {
  port: PortState;
  title: string;
  isPlayer?: boolean;
  onDockShip?: (berthId: number, shipId: number) => void;
  onAssignCrane?: (craneId: number, shipId: number) => void;
}

export const Port: React.FC<PortProps> = ({
  port,
  title,
  isPlayer = false,
  onDockShip,
  onAssignCrane
}) => {
  const [selectedCrane, setSelectedCrane] = React.useState<number | null>(null);

  const waitingShips = port.ships.filter(s => !s.is_docked);
  const dockedShips = port.ships.filter(s => s.is_docked);

  const handleCraneSelect = (craneId: number) => {
    if (!isPlayer) return;
    setSelectedCrane(craneId);
  };

  const handleShipClick = (shipId: number) => {
    if (!isPlayer || !selectedCrane) return;
    const ship = port.ships.find(s => s.id === shipId);
    if (ship && ship.is_docked && onAssignCrane) {
      onAssignCrane(selectedCrane, shipId);
      setSelectedCrane(null);
    }
  };

  return (
    <div className={`port ${isPlayer ? 'player-port' : 'ai-port'}`}>
      <div className="port-header">
        <h2>{title}</h2>
        <div className="port-score">
          Score: <span className="score-value">{port.score}</span>
        </div>
      </div>

      <div className="port-layout">
        {/* Waiting Ships */}
        <div className="section waiting-area">
          <h3>⏳ Waiting Ships ({waitingShips.length})</h3>
          <div className="ships-queue">
            {waitingShips.map(ship => (
              <Ship
                key={ship.id}
                ship={ship}
                isDraggable={isPlayer}
              />
            ))}
            {waitingShips.length === 0 && (
              <div className="empty-message">No ships waiting</div>
            )}
          </div>
        </div>

        {/* Berths */}
        <div className="section berths-area">
          <h3>⚓ Berths</h3>
          <div className="berths-grid">
            {port.berths.map(berth => {
              const ship = dockedShips.find(s => s.docked_at === berth.id);
              return (
                <Berth
                  key={berth.id}
                  berth={berth}
                  ship={ship}
                  onDropShip={isPlayer ? onDockShip : undefined}
                />
              );
            })}
          </div>
        </div>

        {/* Docked Ships (detailed view) */}
        <div className="section docked-area">
          <h3>🚢 Docked Ships ({dockedShips.length})</h3>
          <div className="ships-list">
            {dockedShips.map(ship => (
              <Ship
                key={ship.id}
                ship={ship}
                onDockClick={handleShipClick}
              />
            ))}
            {dockedShips.length === 0 && (
              <div className="empty-message">No ships docked</div>
            )}
          </div>
        </div>

        {/* Cranes */}
        <div className="section cranes-area">
          <h3>🏗️ Cranes</h3>
          <div className="cranes-list">
            {port.cranes.map(crane => (
              <Crane
                key={crane.id}
                crane={crane}
                onAssign={isPlayer ? handleCraneSelect : undefined}
              />
            ))}
          </div>
          {selectedCrane !== null && (
            <div className="crane-selected-hint">
              Crane #{selectedCrane} selected. Click a docked ship to assign.
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
