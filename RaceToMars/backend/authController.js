// controllers/authController.js
const User = require('../models/User');
const jwt = require('jsonwebtoken');

// User registration controller
exports.register = async (req, res) => {
    try {
        const { username, password } = req.body;
        // Validate input and create a new user
        const user = new User({ username, password });
        await user.save();
        res.status(201).json({ message: 'User registered successfully' });
    } catch (err) {
        console.error('Error registering user:', err);
        res.status(500).json({ error: 'User registration failed' });
    }
};

// User login controller
exports.login = async (req, res) => {
    try {
        const { username, password } = req.body;
        // Validate user credentials
        const user = await User.findOne({ username, password });
        if (!user) {
            return res.status(401).json({ error: 'Authentication failed' });
        }

        // Generate and return an authentication token
        const authToken = jwt.sign({ userId: user._id }, 'your_secret_key_here');
        res.json({ authToken });
    } catch (err) {
        console.error('Error authenticating user:', err);
        res.status(401).json({ error: 'Authentication failed' });
    }
};
