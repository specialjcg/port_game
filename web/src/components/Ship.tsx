import React from 'react';
import type { Ship as ShipType } from '../types/game';

interface ShipProps {
  ship: ShipType;
  onDockClick?: (shipId: number) => void;
  isDraggable?: boolean;
}

export const Ship: React.FC<ShipProps> = ({ ship, onDockClick, isDraggable = false }) => {
  const progress = ((ship.containers - ship.containers_remaining) / ship.containers) * 100;

  return (
    <div
      className={`ship ${ship.is_docked ? 'docked' : 'waiting'} ${isDraggable ? 'draggable' : ''}`}
      draggable={isDraggable}
      onDragStart={(e) => {
        e.dataTransfer.setData('shipId', ship.id.toString());
      }}
      onClick={() => onDockClick && onDockClick(ship.id)}
    >
      <div className="ship-header">
        <span className="ship-icon">🚢</span>
        <span className="ship-id">Ship #{ship.id}</span>
      </div>

      <div className="ship-info">
        <div className="container-count">
          {ship.containers_remaining} / {ship.containers} 📦
        </div>

        {ship.assigned_cranes.length > 0 && (
          <div className="cranes-assigned">
            {ship.assigned_cranes.map(craneId => (
              <span key={craneId} className="crane-badge">🏗️ #{craneId}</span>
            ))}
          </div>
        )}

        {ship.is_docked && (
          <div className="progress-bar">
            <div className="progress-fill" style={{ width: `${progress}%` }}></div>
          </div>
        )}
      </div>
    </div>
  );
};
