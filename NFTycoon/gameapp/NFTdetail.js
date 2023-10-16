// NFTDetail.js
import React from 'react';

const NFTDetail = ({ nft }) => {
  return (
    <div>
      <h2>NFT Details</h2>
      <img src={nft.imageUrl} alt={nft.name} />
      <p>Name: {nft.name}</p>
      <p>Description: {nft.description}</p>
      <p>Owner: {nft.owner}</p>
      <p>Price: {nft.price} ETH</p>
      <button>Buy</button>
    </div>
  );
};

export default NFTDetail;
