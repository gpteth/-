import React from 'react';

const Membership = ({ membershipData }) => {
  return (
    <div className="membership">
      <h2>{membershipData.title}</h2>
      <p>{membershipData.description}</p>
      <p>Price: {membershipData.price} ETH</p>
      <button>Subscribe</button>
    </div>
  );
}

export default Membership;
