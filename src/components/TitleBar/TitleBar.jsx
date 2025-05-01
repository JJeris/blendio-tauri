import React from 'react';
import { Link } from 'react-router-dom';
import './TitleBar.css';

const TitleBar = () => {
  return (
    <div className="titlebar flex items-center justify-between p-2 bg-gray-800 text-white">
      <span className="title font-bold text-lg">Blendio-Tauri</span>
      <div className="space-x-4">
        <Link to="/" className="hover:underline">Home</Link>
        <Link to="/blenderdownload" className="hover:underline">Blender Downloads</Link>
        <Link to="/settings" className="hover:underline">Settings</Link>
      </div>
    </div>
  );
};

export default TitleBar;
