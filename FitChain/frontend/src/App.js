import React from 'react';
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';
import Dashboard from './components/Dashboard';
import Profile from './components/Profile';
import Challenge from './components/Challenge';

function App() {
  return (
    <Router>
      <Switch>
        <Route path="/dashboard" component={Dashboard} />
        <Route path="/profile" component={Profile} />
        <Route path="/challenges" component={Challenge} />
        {/* Add more routes */}
      </Switch>
    </Router>
  );
}

export default App;
