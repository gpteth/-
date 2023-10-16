import React, { useState, useEffect } from 'react';
import axios from 'axios';

function Dashboard() {
  const [userData, setUserData] = useState({});
  const [challenges, setChallenges] = useState([]);

  useEffect(() => {
    // Fetch user data and challenges from the backend API
    axios.get('/api/user-data').then((response) => {
      setUserData(response.data);
    });

    axios.get('/api/challenges').then((response) => {
      setChallenges(response.data);
    });
  }, []);

  return (
    <div>
      <h1>Dashboard</h1>
      <h2>Welcome, {userData.username}</h2>
      {/* Display user data */}
      <p>Age: {userData.age}</p>
      <p>Email: {userData.email}</p>

      {/* Display challenges */}
      <h3>Challenges</h3>
      <ul>
        {challenges.map((challenge) => (
          <li key={challenge.id}>{challenge.name}</li>
        ))}
      </ul>
    </div>
  );
}

export default Dashboard;
