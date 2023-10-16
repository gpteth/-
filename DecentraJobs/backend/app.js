const express = require('express');
const mongoose = require('mongoose');
const cors = require('cors');
const jwt = require('jsonwebtoken');
const app = express();
const port = process.env.PORT || 3001;
const apiRouter = require('./routes/api');
const User = require('./models/User');

app.use(cors());
app.use(express.json());

// Connect to MongoDB (replace 'your-mongodb-uri' with your actual MongoDB URI)
mongoose.connect(process.env.MONGODB_URI || 'mongodb://localhost/decentrajobs', {
  useNewUrlParser: true,
  useUnifiedTopology: true,
});

app.use('/api', apiRouter);

// User authentication middleware
const authenticateUser = async (req, res, next) => {
  const token = req.header('Authorization');
  if (!token) {
    return res.status(401).json({ error: 'Authentication required' });
  }

  try {
    const decoded = jwt.verify(token, process.env.JWT_SECRET);
    const user = await User.findById(decoded.userId);
    if (!user) {
      return res.status(401).json({ error: 'Authentication failed' });
    }
    req.user = user;
    next();
  } catch (error) {
    console.error(error);
    res.status(401).json({ error: 'Authentication failed' });
  }
};

app.listen(port, () => {
  console.log(`Server is running on port ${port}`);
});
