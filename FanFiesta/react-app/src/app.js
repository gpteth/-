import React from 'react';
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';
import Header from './components/Header';
import Home from './pages/Home';
import MembershipPage from './pages/MembershipPage';

const App = () => {
  return (
    <Router>
      <div className="App">
        <Header />
        <Switch>
          <Route path="/" exact component={Home} />
          <Route path="/membership" component={MembershipPage} />
          {/* Add more routes for other pages */}
        </Switch>
      </div>
    </Router>
  );
}

export default App;
