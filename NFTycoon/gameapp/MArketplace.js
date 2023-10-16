// Marketplace.js
import React, { useState, useEffect } from 'react';

const Marketplace = () => {
  const [nfts, setNFTs] = useState([]);

  useEffect(() => {
    // Fetch NFTs available for trade from the backend
  }, []);

  return (
    <div>
      <h2>Marketplace</h2>
      <div className="nft-grid">
        {nfts.map((nft) => (
          <div key={nft.id}>
            <img src={nft.imageUrl} alt={nft.name} />
            <p>{nft.name}</p>
            <p>Owner: {nft.owner}</p>
            <button>Trade</button>
          </div>
        ))}
      </div>
    </div>
  );
};

export default Marketplace;
