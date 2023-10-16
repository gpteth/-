const express = require('express');
const app = express();
const passport = require('passport');
const sequelize = require('./config/database');
const authRoutes = require('./routes/authRoutes');

// Initialize Passport
require('./config/passport')(passport);

// Express Middleware
app.use(express.json());
app.use(passport.initialize());

// Routes
app.use('/auth', authRoutes);

// Database synchronization
sequelize
  .sync()
  .then(() => {
    console.log('Database connected');
  })
  .catch((err) => {
    console.error('Database connection error:', err);
  });

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
  console.log(`Server is running on port ${PORT}`);
});
