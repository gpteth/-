import React from 'react';
import Membership from '../components/Membership';

const MembershipPage = () => {
  // Sample membership data
  const memberships = [
    {
      id: 1,
      title: 'Bronze Membership',
      description: 'Basic access to exclusive content',
      price: 0.1,
    },
    // Add more membership tiers here
  ];

  return (
    <div className="membership-page">
      <h1>Membership Tiers</h1>
      <div className="membership-list">
        {memberships.map((membership) => (
          <Membership key={membership.id} membershipData={membership} />
        ))}
      </div>
    </div>
  );
}

export default MembershipPage;
