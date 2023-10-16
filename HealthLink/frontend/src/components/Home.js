import React, { useState } from 'react';

function Home() {
  const [selectedStrategy, setSelectedStrategy] = useState('');
  const [isAutoTradingEnabled, setIsAutoTradingEnabled] = useState(false);

  const handleStrategyChange = (event) => {
    setSelectedStrategy(event.target.value);
  };

  const handleAutoTradingToggle = () => {
    setIsAutoTradingEnabled(!isAutoTradingEnabled);
  };

  const executeTrade = () => {
    // Implement trade execution logic here
    alert(`Executing trade for strategy: ${selectedStrategy}`);
  };

  return (
    <div>
      <h1>WealthWave DeFi Platform</h1>
      <div>
        <h2>AI-Driven Investment Strategies</h2>
        <label>Select a Strategy:</label>
        <select onChange={handleStrategyChange}>
          <option value="strategy1">Strategy 1</option>
          <option value="strategy2">Strategy 2</option>
          {/* Add more strategy options */}
        </select>
      </div>
      <div>
        <h2>Automation</h2>
        <label>
          <input
            type="checkbox"
            checked={isAutoTradingEnabled}
            onChange={handleAutoTradingToggle}
          />
          Enable Auto Trading
        </label>
      </div>
      <button onClick={executeTrade}>Execute Trade</button>
    </div>
  );
}

export default Home;
