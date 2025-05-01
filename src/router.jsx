import React from 'react';
import { Routes, Route } from 'react-router-dom';
import Home from './views/Home';
import Settings from './views/Settings';
import BlenderDownload from './views/BlenderDownload';

const AppRouter = () => (
  <Routes>
    <Route path="/" element={<Home />} />
    <Route path="/blenderdownload" element={<BlenderDownload />} />
    <Route path="/settings" element={<Settings />} />
  </Routes>
);

export default AppRouter;
