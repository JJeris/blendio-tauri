import React, { useState } from 'react';

const Settings = () => {
  const [repoPaths, setRepoPaths] = useState([
    // example data
    {
      id: "1",
      repo_directory_path: "C:\\BlenderRepos\\default",
      is_default: true,
      created: "2025-05-01",
      modified: "2025-05-01",
      accessed: "2025-05-01"
    },
    {
      id: "2",
      repo_directory_path: "D:\\OtherRepo",
      is_default: false,
      created: "2025-04-25",
      modified: "2025-04-28",
      accessed: "2025-04-29"
    }
  ]);
  const [newPath, setNewPath] = useState("");

  const handleAddPath = () => {
    if (!newPath.trim()) return;

    const newEntry = {
      id: Math.random().toString(36).substr(2, 9),
      repo_directory_path: newPath,
      is_default: false,
      created: new Date().toISOString(),
      modified: new Date().toISOString(),
      accessed: new Date().toISOString()
    };

    setRepoPaths([...repoPaths, newEntry]);
    setNewPath("");
  };

  const handleSetDefault = (id) => {
    const updated = repoPaths.map(path => ({
      ...path,
      is_default: path.id === id
    }));
    setRepoPaths(updated);
  };

  return (
    <div className="p-4">
      <h1 className="text-2xl font-bold mb-4">Settings</h1>

      <div className="mb-6">
        <label className="block mb-2 font-medium">New Repo Path:</label>
        <input
          type="text"
          value={newPath}
          onChange={(e) => setNewPath(e.target.value)}
          className="border px-2 py-1 rounded w-full"
        />
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
            <th className="border p-2">Default</th>
            <th className="border p-2">Created</th>
            <th className="border p-2">Modified</th>
            <th className="border p-2">Accessed</th>
          </tr>
        </thead>
        <tbody>
          {repoPaths.map((entry) => (
            <tr key={entry.id}>
              <td className="border p-2">{entry.repo_directory_path}</td>
              <td className="border p-2 text-center">
                <input
                  type="checkbox"
                  checked={entry.is_default}
                  onChange={() => handleSetDefault(entry.id)}
                />
              </td>
              <td className="border p-2">{entry.created}</td>
              <td className="border p-2">{entry.modified}</td>
              <td className="border p-2">{entry.accessed}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default Settings;
