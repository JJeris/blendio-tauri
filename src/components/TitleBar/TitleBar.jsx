import React from 'react';
import { Link } from 'react-router-dom';
import './TitleBar.css';

const TitleBar = () => {
  return (
    <div className="titlebar flex items-center justify-between px-4 py-1 bg-gray-900 text-white">
      <span className="title text-white font-semibold text-base">Blendio-Tauri</span>
      <div className="flex space-x-4 text-sm">
        <Link to="/projectFiles" className="navlink">Project Files</Link>
        <Link to="/installedBlenderVersions" className="navlink">Installed Versions</Link>
        <Link to="/blenderdownload" className="navlink">Downloads</Link>
        <Link to="/settings" className="navlink">Settings</Link>
      </div>
    </div>
  );
};

export default TitleBar;
