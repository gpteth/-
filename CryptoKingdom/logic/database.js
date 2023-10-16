const mongoose = require('mongoose');
mongoose.connect('mongodb://localhost/crypto_kingdom_db', {
    useNewUrlParser: true,
    useUnifiedTopology: true,
});
