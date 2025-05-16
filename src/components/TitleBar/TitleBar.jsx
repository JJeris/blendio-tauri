import React from 'react';
import { Link } from 'react-router-dom';
import './TitleBar.css';

const TitleBar = () => {
  return (
    <div className="titlebar">
      <span>Blendio-Tauri</span>
      <div>
        <Link to="/projectFiles" className="navlink">Project Files</Link>
        <Link to="/installedBlenderVersions" className="navlink">Installed Versions</Link>
        <Link to="/blenderdownload" className="navlink">Downloads</Link>
        <Link to="/settings" className="navlink">Settings</Link>
      </div>
    </div>
  );
};

export default TitleBar;
