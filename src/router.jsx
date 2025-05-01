import React from 'react';
import { Routes, Route } from 'react-router-dom';
import Home from './views/Home';
import Settings from './views/Settings';
import BlenderDownload from './views/BlenderDownload';
import DownloadPopup from './popup/DownloadPopup';

const AppRouter = () => (
  <Routes>
    <Route path="/" element={<Home />} />
    <Route path="/blenderdownload" element={<BlenderDownload />} />
    <Route path="/settings" element={<Settings />} />
    <Route path="/popup" element={<DownloadPopup />} />
  </Routes>
);

export default AppRouter;
