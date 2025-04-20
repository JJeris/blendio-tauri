import React from 'react';
import { BrowserRouter as Router } from 'react-router-dom';
import AppRouter from './router';
import TitleBar from './components/titleBar/TitleBar';

const App = () => {
  return (
    <Router>
      <TitleBar />
      <AppRouter />
    </Router>
  );
};

export default App;
