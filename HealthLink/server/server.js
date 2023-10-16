const express = require('express');
const app = express();
const bodyParser = require('body-parser');
const cors = require('cors');
const mongoose = require('mongoose');

// Middleware
app.use(bodyParser.json());
app.use(cors());

// Connect to MongoDB (assuming you're using it for user data storage)
mongoose.connect('mongodb://localhost/wealthwave', { useNewUrlParser: true, useUnifiedTopology: true });
const db = mongoose.connection;
db.on('error', console.error.bind(console, 'MongoDB connection error:'));
db.once('open', () => {
  console.log('Connected to MongoDB');
});

// Define routes and controllers
const ethereumRoutes = require('./routes/ethereum');
const userRoutes = require('./routes/user');

app.use('/ethereum', ethereumRoutes);
app.use('/user', userRoutes);

const PORT = process.env.PORT || 3001;
app.listen(PORT, () => {
  console.log(`Server is running on port ${PORT}`);
});
