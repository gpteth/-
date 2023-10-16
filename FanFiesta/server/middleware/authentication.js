const passport = require('passport');

// Passport authentication middleware
exports.authenticate = passport.authenticate('jwt', { session: false });
