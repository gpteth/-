// backend/index.js
const express = require('express');
const bodyParser = require('body-parser');
const mongoose = require('mongoose');

const app = express();
const port = process.env.PORT || 3000;

// Replace with your MongoDB connection string
const dbURI = 'mongodb://localhost/racetomars';

// Connect to the MongoDB database
mongoose.connect(dbURI, { useNewUrlParser: true, useUnifiedTopology: true })
    .then(() => {
        console.log('Connected to MongoDB');
    })
    .catch((err) => {
        console.error('Error connecting to MongoDB:', err);
    });

// Define Mongoose schemas and models for User and Spacecraft (replace with your actual schemas)

const User = mongoose.model('User', {
    username: String,
    password: String,
});

const Spacecraft = mongoose.model('Spacecraft', {
    name: String,
    parts: Number,
    isOnMars: Boolean,
    ownerId: mongoose.Schema.Types.ObjectId,
});

app.use(bodyParser.json());

// User registration route
app.post('/api/register', async (req, res) => {
    try {
        const { username, password } = req.body;
        // Validate input (e.g., check for duplicate usernames)

        const user = new User({ username, password });
        await user.save();
        res.status(201).json({ message: 'User registered successfully' });
    } catch (err) {
        console.error('Error registering user:', err);
        res.status(500).json({ error: 'User registration failed' });
    }
});

// User login route
app.post('/api/login', async (req, res) => {
    try {
        const { username, password } = req.body;
        // Validate user credentials (e.g., compare password hashes)

        // Generate and return an authentication token (replace with your authentication logic)
        const authToken = 'your_auth_token_here';
        res.json({ authToken });
    } catch (err) {
        console.error('Error authenticating user:', err);
        res.status(401).json({ error: 'Authentication failed' });
    }
});

// Create a new spacecraft route
app.post('/api/spacecrafts', async (req, res) => {
    try {
        const { name } = req.body;
        const ownerId = req.user.id; // Assuming you have middleware to authenticate users

        // Create a new spacecraft and save it to the database
        const spacecraft = new Spacecraft({ name, parts: 0, isOnMars: false, ownerId });
        await spacecraft.save();

        res.status(201).json(spacecraft);
    } catch (err) {
        console.error('Error creating spacecraft:', err);
        res.status(500).json({ error: 'Spacecraft creation failed' });
    }
});

// Launch a spacecraft to Mars route
app.post('/api/spacecrafts/:id/launch', async (req, res) => {
    try {
        const spacecraftId = req.params.id;
        const ownerId = req.user.id; // Assuming you have middleware to authenticate users

        // Check if the user owns the spacecraft and it's not already on Mars
        const spacecraft = await Spacecraft.findOne({ _id: spacecraftId, ownerId, isOnMars: false });
        if (!spacecraft) {
            return res.status(403).json({ error: 'Permission denied' });
        }

        // Implement logic to launch the spacecraft to Mars (e.g., deduct parts)

        // Update the spacecraft status
        spacecraft.isOnMars = true;
        await spacecraft.save();

        res.json({ message: 'Spacecraft launched to Mars' });
    } catch (err) {
        console.error('Error launching spacecraft:', err);
        res.status(500).json({ error: 'Spacecraft launch failed' });
    }
});

app.listen(port, () => {
    console.log(`Server is running on port ${port}`);
});
