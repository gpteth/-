const jwt = require('jsonwebtoken');

function authenticate(req, res, next) {
  const token = req.header('Authorization');

  if (!token) {
    return res.status(401).json({ error: 'Unauthorized' });
  }

  try {
    // Verify the JWT token
    const decoded = jwt.verify(token, 'your-secret-key');
    req.user = decoded;
    next();
  } catch (error) {
    console.error(error);
    res.status(401).json({ error: 'Token is invalid' });
  }
}

module.exports = {
  authenticate,
};
