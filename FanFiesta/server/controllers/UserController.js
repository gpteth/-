const User = require('../models/User');

// User registration
exports.registerUser = async (req, res) => {
  try {
    // Create a new user
    const user = await User.create(req.body);
    // Send a success response
    res.status(201).json({ message: 'User registered successfully', user });
  } catch (error) {
    // Handle error
    res.status(500).json({ error: 'Internal server error' });
  }
};
