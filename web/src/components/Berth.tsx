import React from 'react';
import type { Berth as BerthType, Ship } from '../types/game';

interface BerthProps {
  berth: BerthType;
  ship?: Ship;
  onDropShip?: (berthId: number, shipId: number) => void;
}

export const Berth: React.FC<BerthProps> = ({ berth, ship, onDropShip }) => {
  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    const shipId = parseInt(e.dataTransfer.getData('shipId'));
    if (onDropShip && berth.is_free) {
      onDropShip(shipId, berth.id);
    }
  };

  const handleDragOver = (e: React.DragEvent) => {
    if (berth.is_free) {
      e.preventDefault();
    }
  };

  // Déterminer si le quai est réellement occupé (il faut un berth non libre ET un navire valide)
  const isOccupied = !berth.is_free && ship && ship.containers_remaining > 0;

  return (
    <div
      className={`berth ${isOccupied ? 'occupied' : 'free'}`}
      onDrop={handleDrop}
      onDragOver={handleDragOver}
    >
      <div className="berth-header">
        <span className="berth-icon">⚓</span>
        <span className="berth-id">Berth #{berth.id}</span>
      </div>

      <div className="berth-content">
        {!isOccupied ? (
          <div className="berth-empty">
            <span className="drop-hint">Drop ship here</span>
          </div>
        ) : ship ? (
          <div className="berth-ship">
            <span className="ship-icon">🚢</span>
            <span>Ship #{ship.id}</span>
            <div className="container-info">
              {ship.containers_remaining} / {ship.containers} 📦
            </div>
          </div>
        ) : null}
      </div>
    </div>
  );
};
