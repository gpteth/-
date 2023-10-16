import React from 'react';
import { BrowserRouter as Router, Route, Switch, Link } from 'react-router-dom';
import ArtworkList from './components/ArtworkList';
import MintArtworkForm from './components/MintArtworkForm';
import Auction from './components/Auction';
import UserProfile from './components/UserProfile';
import EditArtwork from './components/EditArtwork';

function App() {
  return (
    <Router>
      <div>
        <nav>
          <ul>
            <li>
              <Link to="/">Home</Link>
            </li>
            <li>
              <Link to="/mint">Mint Artwork</Link>
            </li>
            <li>
              <Link to="/profile">My Profile</Link>
            </li>
          </ul>
        </nav>

        <Switch>
          <Route path="/" exact component={ArtworkList} />
          <Route path="/mint" component={MintArtworkForm} />
          <Route path="/auction/:id" component={Auction} />
          <Route path="/profile" component={UserProfile} />
          <Route path="/edit/:id" component={EditArtwork} />
        </Switch>
      </div>
    </Router>
  );
}

export default App;
