import React, { useState, useEffect } from 'react';

function Auction() {
  const [auctionDetails, setAuctionDetails] = useState(null);

  // Fetch auction details from your backend API and update the state
  useEffect(() => {
    fetch('/api/auctions/1') // Replace '1' with the actual auction ID
      .then((response) => response.json())
      .then((data) => setAuctionDetails(data));
  }, []);

  return (
    <div>
      {auctionDetails ? (
        <div>
          <h2>Auction Details</h2>
          <p>Title: {auctionDetails.title}</p>
          <p>Description: {auctionDetails.description}</p>
          <p>Current Bid: {auctionDetails.currentBid}</p>
        </div>
      ) : (
        <p>Loading auction details...</p>
      )}
    </div>
  );
}

export default Auction;
