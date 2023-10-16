import React from 'react';

function Portfolio({ portfolio }) {
  return (
    <div className="portfolio">
      <h2>Your Portfolio</h2>
      <ul>
        {portfolio.map((asset) => (
          <li key={asset.id}>
            {asset.name}: {asset.amount}
          </li>
        ))}
      </ul>
    </div>
  );
}

export default Portfolio;
