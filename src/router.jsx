import React from 'react';
import { Routes, Route } from 'react-router-dom';
import Home from './views/Home';
import Settings from './views/Settings';

const AppRouter = () => (
  <Routes>
    <Route path="/" element={<Home />} />
    <Route path="/settings" element={<Settings />} />
  </Routes>
);

export default AppRouter;
