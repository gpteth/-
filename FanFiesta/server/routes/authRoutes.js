const express = require('express');
const router = express.Router();
const UserController = require('../controllers/UserController');
const authenticationMiddleware = require('../middleware/authentication');

// User registration route
router.post('/register', UserController.registerUser);

// Protected route (requires authentication)
router.get('/protected', authenticationMiddleware.authenticate, (req, res) => {
  res.json({ message: 'This is a protected route' });
});

module.exports = router;
