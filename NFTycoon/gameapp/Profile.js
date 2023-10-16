// Profile.js
import React, { useState } from 'react';

const Profile = ({ user }) => {
  const [businesses, setBusinesses] = useState([]);
  const [selectedBusiness, setSelectedBusiness] = useState(null);

  const selectBusiness = (business) => {
    // Set the selected business and fetch its products
  };

  return (
    <div>
      <h2>User Profile</h2>
      <p>Username: {user.username}</p>
      <h3>My Businesses</h3>
      <ul>
        {businesses.map((business) => (
          <li key={business.id} onClick={() => selectBusiness(business)}>
            {business.name}
          </li>
        ))}
      </ul>
      {selectedBusiness && (
        <div>
          <h3>Manage Products</h3>
          {/* Display and manage products for the selected business */}
        </div>
      )}
    </div>
  );
};

export default Profile;
