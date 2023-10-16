import React, { useState, useEffect } from 'react';
import axios from 'axios';

function Profile() {
  const [userProfile, setUserProfile] = useState({});

  useEffect(() => {
    // Fetch user profile data from the backend API
    axios.get('/api/profile').then((response) => {
      setUserProfile(response.data);
    });
  }, []);

  return (
    <div>
      <h1>Profile</h1>
      <p>Username: {userProfile.username}</p>
      <p>Age: {userProfile.age}</p>
      <p>Email: {userProfile.email}</p>
      {/* Display other profile information */}
    </div>
  );
}

export default Profile;
