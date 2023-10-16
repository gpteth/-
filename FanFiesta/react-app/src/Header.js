import React from 'react';
import { Link } from 'react-router-dom';

const Header = () => {
  return (
    <header>
      <nav>
        <ul>
          <li><Link to="/">Home</Link></li>
          <li><Link to="/artist-profile">Artist Profile</Link></li>
          <li><Link to="/membership">Memberships</Link></li>
          <li><Link to="/exclusive-content">Exclusive Content</Link></li>
          <li><Link to="/virtual-events">Virtual Events</Link></li>
          <li><Link to="/wallet">Wallet</Link></li>
        </ul>
      </nav>
    </header>
  );
}

export default Header;
