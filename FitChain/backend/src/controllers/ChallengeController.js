const express = require('express');
const router = express.Router();
const { Challenge } = require('../utils/database'); // Import your database models

// Create a new challenge
router.post('/', async (req, res) => {
  try {
    const { name, description, reward } = req.body;

    // Create a new challenge
    const newChallenge = await Challenge.create({
      name,
      description,
      reward,
    });

    res.status(201).json(newChallenge);
  } catch (error) {
    console.error(error);
    res.status(500).json({ error: 'Server error' });
  }
});

// Get all challenges
router.get('/', async (req, res) => {
  try {
    // Fetch all challenges from the database
    const challenges = await Challenge.findAll();

    res.json(challenges);
  } catch (error) {
    console.error(error);
    res.status(500).json({ error: 'Server error' });
  }
});

// Join a challenge
router.post('/:challengeId/join', async (req, res) => {
  try {
    const { challengeId } = req.params;

    // Check if the challenge exists
    const challenge = await Challenge.findByPk(challengeId);
    if (!challenge) {
      return res.status(404).json({ error: 'Challenge not found' });
    }

    // Implement logic to allow the user to join the challenge
    // You may need to update the user's data in the database

    res.json({ message: 'Joined the challenge successfully' });
  } catch (error) {
    console.error(error);
    res.status(500).json({ error: 'Server error' });
  }
});

module.exports = router;
