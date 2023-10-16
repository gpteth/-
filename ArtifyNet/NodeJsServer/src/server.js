const express = require('express');
const mongoose = require('mongoose');
const bodyParser = require('body-parser');
const cors = require('cors');
const passport = require('passport');
const LocalStrategy = require('passport-local').Strategy;
const session = require('express-session');
const bcrypt = require('bcrypt');
const User = require('./models/User');
const artworkRoutes = require('./routes/artworkRoutes');
const userRoutes = require('./routes/userRoutes');
const auctionRoutes = require('./routes/auctionRoutes');

const app = express();
const port = process.env.PORT || 3000;

// Connect to MongoDB using Mongoose
mongoose.connect('mongodb://localhost/artifynet', {
  useNewUrlParser: true,
  useUnifiedTopology: true,
});

// Middleware
app.use(bodyParser.json());
app.use(cors()); // Enable CORS for all routes (configure origin as needed)

// Configure express-session for user authentication
app.use(
  session({
    secret: 'your-secret-key',
    resave: false,
    saveUninitialized: false,
  })
);

// Initialize passport
app.use(passport.initialize());
app.use(passport.session());

// Passport local strategy for user authentication
passport.use(
  new LocalStrategy(async (username, password, done) => {
    try {
      const user = await User.findOne({ username });
      if (!user) return done(null, false);
      if (await bcrypt.compare(password, user.password)) {
        return done(null, user);
      } else {
        return done(null, false);
      }
    } catch (error) {
      return done(error);
    }
  })
);

// Passport serialize and deserialize user
passport.serializeUser((user, done) => {
  done(null, user.id);
});

passport.deserializeUser(async (id, done) => {
  try {
    const user = await User.findById(id);
    done(null, user);
  } catch (error) {
    done(error);
  }
});

// Define API routes
app.use('/api', artworkRoutes);
app.use('/api', userRoutes);
app.use('/api', auctionRoutes);

// Error handling middleware
app.use((err, req, res, next) => {
  console.error(err.stack);
  res.status(500).json({ error: 'Internal Server Error' });
});

// Start the server
app.listen(port, () => {
  console.log(`Server is running on port ${port}`);
});
