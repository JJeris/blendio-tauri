import React from 'react';
import { Routes, Route } from 'react-router-dom';
import Settings from './views/Settings';
import BlenderDownload from './views/BlenderDownload';
import DownloadPopup from './popup/DownloadPopup';
import InstalledBlenderVersions from './views/InstalledBlenderVersions';
import ProjectFiles from './views/ProjectFiles';
import CreateBlendPopup from './popup/CreateBlendPopup';
import LaunchBlendPopup from './popup/LaunchBlendPopup';
import LaunchBlenderPopup from './popup/LaunchBlenderPopup';

const AppRouter = () => (
  <Routes>
    <Route path="/" element={<ProjectFiles />} />
    <Route path="/projectFiles" element={<ProjectFiles />} />
    <Route path="/installedBlenderVersions" element={<InstalledBlenderVersions />} />
    <Route path="/blenderdownload" element={<BlenderDownload />} />
    <Route path="/settings" element={<Settings />} />
    <Route path="/popup/DownloadPopup" element={<DownloadPopup />} />
    <Route path="/popup/CreateBlendPopup" element={<CreateBlendPopup />} />
    <Route path="/popup/LaunchBlendPopup" element={<LaunchBlendPopup />} />
    <Route path="/popup/LaunchBlenderPopup" element={<LaunchBlenderPopup />} />

  </Routes>
);

export default AppRouter;
