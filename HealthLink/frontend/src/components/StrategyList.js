import React from 'react';
import StrategyCard from './StrategyCard';

function StrategyList({ strategies }) {
  return (
    <div className="strategy-list">
      <h2>Available Investment Strategies</h2>
      {strategies.map((strategy) => (
        <StrategyCard key={strategy.id} strategy={strategy} />
      ))}
    </div>
  );
}

export default StrategyList;
