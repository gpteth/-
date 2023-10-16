const { Sequelize } = require('sequelize');

const sequelize = new Sequelize({
  dialect: 'sqlite',
  storage: 'database.sqlite', // Change to your preferred database
});

const User = sequelize.define('User', {
  username: {
    type: Sequelize.STRING,
    allowNull: false,
  },
  email: {
    type: Sequelize.STRING,
    allowNull: false,
    unique: true,
  },
  password: {
    type: Sequelize.STRING,
    allowNull: false,
  },
});

const Challenge = sequelize.define('Challenge', {
  name: {
    type: Sequelize.STRING,
    allowNull: false,
  },
  description: {
    type: Sequelize.TEXT,
  },
  reward: {
    type: Sequelize.INTEGER,
  },
});

module.exports = {
  sequelize,
  User,
  Challenge,
};

