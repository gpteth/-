const express = require('express');
const app = express();
const cors = require('cors'); // For handling Cross-Origin Resource Sharing (CORS)
const { sequelize } = require('./utils/database'); // Import your database connection
const { errorHandler } = require('./middleware/errorHandlingMiddleware'); // Import error handling middleware
const userRoutes = require('./routes/userRoutes');
const challengeRoutes = require('./routes/challengeRoutes');

app.use(cors()); // Enable CORS for your frontend

// Body parsing middleware
app.use(express.json());

// Use the user and challenge routes
app.use('/api/users', userRoutes);
app.use('/api/challenges', challengeRoutes);

// Error handling middleware
app.use(errorHandler);

// Sync the database and start the server
sequelize.sync().then(() => {
  const PORT = process.env.PORT || 3001;
  app.listen(PORT, () => {
    console.log(`Server is running on port ${PORT}`);
  });
});
