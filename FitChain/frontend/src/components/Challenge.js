import React, { useState, useEffect } from 'react';
import axios from 'axios';

function Challenge() {
  const [challenges, setChallenges] = useState([]);

  useEffect(() => {
    // Fetch available challenges from the backend API
    axios.get('/api/challenges').then((response) => {
      setChallenges(response.data);
    });
  }, []);

  const joinChallenge = (challengeId) => {
    // Implement logic to allow users to join a challenge
    // Send a request to the backend to join the challenge
    axios.post(`/api/challenges/${challengeId}/join`).then((response) => {
      // Handle success or error
    });
  };

  return (
    <div>
      <h1>Challenges</h1>
      <ul>
        {challenges.map((challenge) => (
          <li key={challenge.id}>
            {challenge.name}{' '}
            <button onClick={() => joinChallenge(challenge.id)}>Join</button>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default Challenge;
