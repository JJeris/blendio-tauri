import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

const Settings = () => {
  const [repoPaths, setRepoPaths] = useState([]);

  useEffect(() => {
    loadPaths();
  }, []);

  const loadPaths = async () => {
    try {
      const paths = await invoke("fetch_blender_version_installation_locations", {
        id: null,
        limit: null,
      });
      setRepoPaths(paths);
    } catch (error) {
      console.error("Failed to fetch paths:", error);
    }
  };

  const handleAddPath = async () => {
    try {
      await invoke("insert_blender_version_installation_location");
      await loadPaths();
    } catch (error) {
      console.error("Failed to insert path:", error);
    }
  };

  const handleSetDefault = async (selectedId) => {
    try {
      await invoke("update_blender_version_installation_location", {
        id: selectedId,
        repoDirectoryPath: repoPaths.find((e) => e.id === selectedId).repo_directory_path,
        isDefault: repoPaths.find((e) => e.id === selectedId).is_default,
      });
      await loadPaths();
    } catch (error) {
      console.error("Failed to update default status:", error);
    }
  };

  const handleDelete = async (id) => {
    try {
      await invoke("delete_blender_version_installation_location", { id });
      await loadPaths();
    } catch (error) {
      console.error("Failed to delete path:", error);
    }
  };

  return (
    <div className="p-4">
      <h1 className="text-2xl font-bold mb-4">Settings</h1>

      <div className="mb-6">
        <button
          className="mt-2 bg-green-500 text-white px-4 py-2 rounded"
          onClick={handleAddPath}
        >
          Add Path
        </button>
      </div>

      <table className="w-full border-collapse border text-sm">
        <thead>
          <tr>
            <th className="border p-2">Path</th>
            <th className="border p-2">Created</th>
            <th className="border p-2">Modified</th>
            <th className="border p-2">Accessed</th>
            <th className="border p-2">Default</th>
            <th className="border p-2">Actions</th>
          </tr>
        </thead>
        <tbody>
          {repoPaths.map((entry) => (
            <tr key={entry.id}>
              <td className="border p-2">{entry.repo_directory_path}</td>
              <td className="border p-2">{entry.created}</td>
              <td className="border p-2">{entry.modified}</td>
              <td className="border p-2">{entry.accessed}</td>
              <td className="border p-2 text-center">
                <input
                  type="checkbox"
                  checked={entry.is_default}
                  onChange={() => handleSetDefault(entry.id)}
                />
              </td>
              <td className="border p-2 text-center">
                <button
                  className="text-red-500 hover:underline"
                  onClick={() => handleDelete(entry.id)}
                >
                  Delete
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default Settings;
