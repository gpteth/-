import React, { useState, useEffect } from 'react';
import { BrowserRouter as Router, Route, Switch, Link, Redirect } from 'react-router-dom';
import Home from './components/Home';
import StrategyList from './components/StrategyList';
import Portfolio from './components/Portfolio';
import StrategyDetail from './components/StrategyDetail'; // Assuming you have a detailed strategy view
import Login from './components/Login';
import PrivateRoute from './components/PrivateRoute'; // Implement a PrivateRoute component for protected routes

function App() {
  const [user, setUser] = useState(null);
  const [strategies, setStrategies] = useState([]);
  const [portfolio, setPortfolio] = useState([]);

  // Fetch strategies and portfolio data from your backend
  useEffect(() => {
    // Implement API calls to fetch data and update state
    // Example:
    // fetch('/api/strategies')
    //   .then((response) => response.json())
    //   .then((data) => setStrategies(data));

    // Similarly, fetch user portfolio data
  }, []);

  return (
    <Router>
      <div className="App">
        <nav>
          <ul>
            <li>
              <Link to="/">Home</Link>
            </li>
            <li>
              <Link to="/strategies">Strategies</Link>
            </li>
            <li>
              {user ? (
                <Link to="/portfolio">Portfolio</Link>
              ) : (
                <Link to="/login">Login</Link>
              )}
            </li>
          </ul>
        </nav>

        <Switch>
          <Route path="/login">
            {user ? <Redirect to="/portfolio" /> : <Login setUser={setUser} />}
          </Route>
          <PrivateRoute path="/portfolio" user={user}>
            <Portfolio portfolio={portfolio} />
          </PrivateRoute>
          <Route path="/strategies">
            <StrategyList strategies={strategies} user={user} />
          </Route>
          <Route path="/strategy/:id">
            <StrategyDetail strategies={strategies} user={user} />
          </Route>
          <Route path="/">
            <Home />
          </Route>
        </Switch>
      </div>
    </Router>
  );
}

export default App;
