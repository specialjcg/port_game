import React from 'react';
import type { Crane as CraneType } from '../types/game';

interface CraneProps {
  crane: CraneType;
  onAssign?: (craneId: number) => void;
}

export const Crane: React.FC<CraneProps> = ({ crane, onAssign }) => {
  return (
    <div
      className={`crane ${crane.is_free ? 'free' : 'busy'}`}
      onClick={() => crane.is_free && onAssign && onAssign(crane.id)}
      draggable={crane.is_free}
      onDragStart={(e) => {
        if (crane.is_free) {
          e.dataTransfer.setData('craneId', crane.id.toString());
        }
      }}
    >
      <div className="crane-icon">🏗️</div>
      <div className="crane-info">
        <div className="crane-id">Crane #{crane.id}</div>
        <div className="crane-status">
          {crane.is_free ? (
            <span className="status-free">Available</span>
          ) : (
            <span className="status-busy">
              Working on Ship #{crane.assigned_to}
            </span>
          )}
        </div>
        <div className="crane-speed">
          Speed: {crane.processing_speed}x
        </div>
      </div>
    </div>
  );
};
