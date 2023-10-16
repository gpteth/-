// Header.js
import React from 'react';

const Header = ({ user }) => {
  return (
    <header>
      <nav>
        <ul>
          <li>Home</li>
          <li>Marketplace</li>
          <li>Profile</li>
        </ul>
      </nav>
      {user ? (
        <div>
          <p>Welcome, {user.username}</p>
          <button>Logout</button>
        </div>
      ) : (
        <div>
          <button>Login</button>
          <button>Register</button>
        </div>
      )}
    </header>
  );
};

export default Header;
