// controllers/spacecraftController.js
const Spacecraft = require('../models/Spacecraft');

// Create a new spacecraft controller
exports.createSpacecraft = async (req, res) => {
    try {
        const { name } = req.body;
        const ownerId = req.user.userId;

        // Create a new spacecraft and save it to the database
        const spacecraft = new Spacecraft({ name, parts: 0, isOnMars: false, ownerId });
        await spacecraft.save();

        res.status(201).json(spacecraft);
    } catch (err) {
        console.error('Error creating spacecraft:', err);
        res.status(500).json({ error: 'Spacecraft creation failed' });
    }
};

// Launch a spacecraft to Mars controller
exports.launchToMars = async (req, res) => {
    try {
        const spacecraftId = req.params.id;
        const ownerId = req.user.userId;

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
};
