const express = require('express');
const app = express();
const bodyParser = require('body-parser');
const jwtMiddleware = require('./jwt-middleware'); // Middleware to verify JWT

app.use(bodyParser.json());
app.use(jwtMiddleware); // Use JWT middleware to protect routes that require authentication

// Define routes for game actions (e.g., /buy-land, /explore, /battle, /trade)

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
    console.log(`Server is running on port ${PORT}`);
});
