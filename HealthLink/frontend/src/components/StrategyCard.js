import React from 'react';

function StrategyCard({ strategy }) {
  return (
    <div className="strategy-card">
      <h3>{strategy.name}</h3>
      <p>Description: {strategy.description}</p>
      <p>Performance: {strategy.performance}</p>
      <button>Invest</button>
    </div>
  );
}

export default StrategyCard;
