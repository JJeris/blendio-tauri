import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { emit } from "@tauri-apps/api/event";

const DownloadPopup = () => {
  const [repoPaths, setRepoPaths] = useState([]);

  useEffect(() => {
    const fetchPaths = async () => {
      const paths = await invoke("fetch_blender_version_installation_locations", {
        id: null,
        limit: null,
      });
      setRepoPaths(paths);
    };
    fetchPaths();
  }, []);

  const closeWindow = async () => {
    const appWindow = getCurrentWindow();
    appWindow.close();
  };

  const handleSelect = async (path) => {
    await emit("download-path-selected", { path: path });
    await closeWindow();
  };

  const handleUseDefault = async () => {
    await emit("download-path-selected", { path: repoPaths.find((e) => e.is_default === true).repo_directory_path });
    await closeWindow();
  };

  return (
    <div className="p-4 text-sm">
      <h1 className="text-lg font-bold mb-2">Choose Download Location</h1>
      <button
        className="mb-2 px-4 py-2 bg-blue-500 text-white rounded"
        onClick={handleUseDefault}
      >
        Use Default Directory
      </button>
      Other:
      <ul className="space-y-2 mt-4">
        {repoPaths.map((path) => (
          <li key={path.id}>
            <button
              className="w-full text-left px-2 py-1 border rounded hover:bg-gray-100"
              onClick={() => handleSelect(path.repo_directory_path)}
            >
              {path.repo_directory_path}
            </button>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default DownloadPopup;
