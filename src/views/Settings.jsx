import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

const Settings = () => {
  const [repoPaths, setRepoPaths] = useState([]);
  const [launchArgs, setLaunchArgs] = useState([]);
  const [pythonScripts, setPythonScripts] = useState([]);

  useEffect(() => {
    loadPaths();
    loadLaunchArgs();
    loadPythonScripts();
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

  const loadLaunchArgs = async () => {
    try {
      const args = await invoke("fetch_launch_arguments", {
        id: null,
        limit: null,
      });
      setLaunchArgs(args);
    } catch (error) {
      console.error("Failed to fetch launch arguments:", error);
    }
  };

  const loadPythonScripts = async () => {
    try {
      const scripts = await invoke("fetch_python_scripts", {
        id: null,
        limit: null,
      });
      setPythonScripts(scripts);
    } catch (error) {
      console.error("Failed to fetch python scripts:", error);
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

  const handleSetDefaultBlenderInstallationPaths = async (selectedId) => {
    try {
      await invoke("update_blender_version_installation_location", {
        id: selectedId,
        isDefault: repoPaths.find((e) => e.id === selectedId).is_default,
      });
      await loadPaths();
    } catch (error) {
      console.error("Failed to update default status:", error);
    }
  };

  const handleSetDefaultLaunchArg = async (selectedId) => {
    try {
      await invoke("update_launch_argument", {
        id: selectedId,
        isDefault: launchArgs.find((e) => e.id === selectedId).is_default,
      });
      await loadLaunchArgs();
    } catch (error) {
      console.error("Failed to update default status for launch argument:", error);
    }
  };

  const handleDeleteBlenderVersionInstallationPath = async (id) => {
    try {
      await invoke("delete_blender_version_installation_location", { id });
      await loadPaths();
    } catch (error) {
      console.error("Failed to delete path:", error);
    }
  };

  const handleDeleteLaunchArg = async (id) => {
    try {
      await invoke("delete_launch_argument", { id });
      await loadLaunchArgs();
    } catch (error) {
      console.error("Failed to delete launch argument:", error);
    }
  };

  const handleDeletePythonScript = async (id) => {
    try {
      await invoke("delete_python_script", { id });
      await loadPythonScripts();
    } catch (error) {
      console.error("Failed to delete python script:", error);
    }
  };

  return (
    <div className="p-4">
      <h1 className="text-2xl font-bold mb-4">Settings</h1>
      <h2 className="text-xl font-semibold mt-8 mb-2">Blender installation paths</h2>
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
                  onChange={() => handleSetDefaultBlenderInstallationPaths(entry.id)}
                />
              </td>
              <td className="border p-2 text-center">
                <button
                  className="text-red-500 hover:underline"
                  onClick={() => handleDeleteBlenderVersionInstallationPath(entry.id)}
                >
                  Delete
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>

      <h2 className="text-xl font-semibold mt-8 mb-2">Launch Arguments</h2>
      <table className="w-full border-collapse border text-sm mb-6">
        <thead>
          <tr>
            <th className="border p-2">Argument String</th>
            <th className="border p-2">Created</th>
            <th className="border p-2">Modified</th>
            <th className="border p-2">Accessed</th>
            <th className="border p-2">Default</th>
            <th className="border p-2">Actions</th>
          </tr>
        </thead>
        <tbody>
          {launchArgs.map((arg) => (
            <tr key={arg.id}>
              <td className="border p-2">{arg.argument_string}</td>
              <td className="border p-2">{arg.created}</td>
              <td className="border p-2">{arg.modified}</td>
              <td className="border p-2">{arg.accessed}</td>
              <td className="border p-2 text-center">
                <input
                  type="checkbox"
                  checked={arg.is_default}
                  onChange={() => handleSetDefaultLaunchArg(arg.id)}
                />
              </td>
              <td className="border p-2 text-center">
                <button
                  className="text-red-500 hover:underline"
                  onClick={() => handleDeleteLaunchArg(arg.id)}
                >
                  Delete
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>

      <h2 className="text-xl font-semibold mt-8 mb-2">Python Scripts</h2>
      <table className="w-full border-collapse border text-sm">
        <thead>
          <tr>
            <th className="border p-2">Script Path</th>
            <th className="border p-2">Created</th>
            <th className="border p-2">Modified</th>
            <th className="border p-2">Accessed</th>
            <th className="border p-2">Actions</th>
          </tr>
        </thead>
        <tbody>
          {pythonScripts.map((script) => (
            <tr key={script.id}>
              <td className="border p-2">{script.script_file_path}</td>
              <td className="border p-2">{script.created}</td>
              <td className="border p-2">{script.modified}</td>
              <td className="border p-2">{script.accessed}</td>
              <td className="border p-2 text-center">
                <button
                  className="text-red-500 hover:underline"
                  onClick={() => handleDeletePythonScript(script.id)}
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
