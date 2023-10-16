// backend/main.js
const express = require('express');
const mongoose = require('mongoose');
const config = require('./config/config');
const authRoutes = require('./routes/authRoutes');
const nftRoutes = require('./routes/nftRoutes');
const userRoutes = require('./routes/userRoutes');
const passport = require('passport');
const session = require('express-session');
const cookieParser = require('cookie-parser');
const flash = require('connect-flash');

const app = express();
const port = process.env.PORT || 3000;

// Middleware for parsing JSON requests
app.use(express.json());

// Middleware for parsing cookies
app.use(cookieParser());

// Initialize Passport and session for user authentication
app.use(session({
  secret: 'your-secret-key',
  resave: true,
  saveUninitialized: true,
}));
app.use(passport.initialize());
app.use(passport.session());
app.use(flash());

// MongoDB connection
mongoose.connect(config.mongoURI, { useNewUrlParser: true, useUnifiedTopology: true });
const db = mongoose.connection;
db.on('error', console.error.bind(console, 'MongoDB connection error:'));

// Passport configuration (assuming you use Passport for authentication)
require('./config/passport')(passport);

// Global middleware to make user data available in views
app.use((req, res, next) => {
  res.locals.user = req.user || null;
  next();
});

// API routes
app.use('/api/auth', authRoutes);
app.use('/api/nft', nftRoutes);
app.use('/api/user', userRoutes);

// Serve static files (e.g., for React frontend)
app.use(express.static('public'));

// Error handling middleware
app.use((err, req, res, next) => {
  console.error(err.stack);
  res.status(500).send('Something went wrong!');
});

// Start the server
app.listen(port, () => {
  console.log(`Server is running on port ${port}`);
});
