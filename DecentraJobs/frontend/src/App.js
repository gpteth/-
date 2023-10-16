import React from 'react';
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';
import JobList from './components/JobList';
import JobDetail from './components/JobDetail';
import CreateJob from './components/CreateJob';

function App() {
  return (
    <Router>
      <div className="App">
        <Switch>
          <Route path="/" exact component={JobList} />
          <Route path="/job/:id" component={JobDetail} />
          <Route path="/create" component={CreateJob} />
        </Switch>
      </div>
    </Router>
  );
}

export default App;
