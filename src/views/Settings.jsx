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
                repoDirectoryPath: null
            });
            setRepoPaths(paths);
        } catch (err) {
            setRepoPaths([]);
            console.error("Failed to fetch paths:", err);
        }
    };

    const loadLaunchArgs = async () => {
        try {
            const args = await invoke("fetch_launch_arguments", {
                id: null,
                limit: null,
                argumentString: null
            });
            setLaunchArgs(args);
        } catch (err) {
            setLaunchArgs([]);
            console.error("Failed to fetch launch arguments:", err);
        }
    };

    const loadPythonScripts = async () => {
        try {
            const scripts = await invoke("fetch_python_scripts", {
                id: null,
                limit: null,
                scriptFilePath: null
            });
            setPythonScripts(scripts);
        } catch (err) {
            setPythonScripts([]);
            console.error("Failed to fetch python scripts:", err);
        }
    };

    const handleAddPath = async () => {
        try {
            await invoke("insert_blender_version_installation_location");
            await loadPaths();
        } catch (err) {
            await loadPaths();
            console.error("Failed to insert path:", err);
        }
    };

    const handleSetDefaultBlenderInstallationPaths = async (selectedId) => {
        try {
            await invoke("update_blender_version_installation_location", {
                id: selectedId,
                isDefault: repoPaths.find((e) => e.id === selectedId).is_default,
            });
            await loadPaths();
        } catch (err) {
            await loadPaths();
            console.error("Failed to update default status:", err);
        }
    };

    const handleSetDefaultLaunchArg = async (selectedId) => {
        try {
            await invoke("update_launch_argument", {
                id: selectedId,
                isDefault: launchArgs.find((e) => e.id === selectedId).is_default,
            });
            await loadLaunchArgs();
        } catch (err) {
            await loadLaunchArgs();
            console.error("Failed to update default status for launch argument:", err);
        }
    };

    const handleDeleteBlenderVersionInstallationPath = async (id) => {
        try {
            await invoke("delete_blender_version_installation_location", { id });
            await loadPaths();
        } catch (err) {
            await loadPaths();
            console.error("Failed to delete path:", err);
        }
    };

    const handleDeleteLaunchArg = async (id) => {
        try {
            await invoke("delete_launch_argument", { id });
            await loadLaunchArgs();
        } catch (err) {
            await loadLaunchArgs();
            console.error("Failed to delete launch argument:", err);
        }
    };

    const handleDeletePythonScript = async (id) => {
        try {
            await invoke("delete_python_script", { id });
            await loadPythonScripts();
        } catch (err) {
            await loadPythonScripts();
            console.error("Failed to delete python script:", err);
        }
    };

    return (
        <div className="p-4">
            <h1 className="mb-4">Settings</h1>
            <h2 className="mt-8 mb-2">Blender installation paths</h2>
            <div className="mb-6">
                <button
                    className="mt-2 bg-green-500"
                    onClick={handleAddPath}
                >
                    Add Path
                </button>
            </div>

            <table className="border-collapse">
                <thead>
                    <tr>
                        <th className="p-2">Directory path</th>
                        <th className="p-2">Created</th>
                        <th className="p-2">Modified</th>
                        <th className="p-2">Accessed</th>
                        <th className="p-2">Default</th>
                        <th className="p-2">Actions</th>
                    </tr>
                </thead>
                <tbody>
                    {repoPaths.map((entry) => (
                        <tr key={entry.id}>
                            <td className="p-2">{entry.repo_directory_path}</td>
                            <td className="p-2">{entry.created}</td>
                            <td className="p-2">{entry.modified}</td>
                            <td className="p-2">{entry.accessed}</td>
                            <td className="p-2">
                                <input
                                    type="checkbox"
                                    checked={entry.is_default}
                                    onChange={() => handleSetDefaultBlenderInstallationPaths(entry.id)}
                                />
                            </td>
                            <td className="p-2">
                                <button
                                    className="text-red-500 "
                                    onClick={() => handleDeleteBlenderVersionInstallationPath(entry.id)}
                                >
                                    Delete
                                </button>
                            </td>
                        </tr>
                    ))}
                </tbody>
            </table>

            <h2 className="mt-8 mb-2">Launch Arguments</h2>
            <table className="border-collapse mb-6">
                <thead>
                    <tr>
                        <th className="p-2">Argument String</th>
                        <th className="p-2">Created</th>
                        <th className="p-2">Modified</th>
                        <th className="p-2">Accessed</th>
                        <th className="p-2">Default</th>
                        <th className="p-2">Actions</th>
                    </tr>
                </thead>
                <tbody>
                    {launchArgs.map((arg) => (
                        <tr key={arg.id}>
                            <td className="p-2">{arg.argument_string}</td>
                            <td className="p-2">{arg.created}</td>
                            <td className="p-2">{arg.modified}</td>
                            <td className="p-2">{arg.accessed}</td>
                            <td className="p-2">
                                <input
                                    type="checkbox"
                                    checked={arg.is_default}
                                    onChange={() => handleSetDefaultLaunchArg(arg.id)}
                                />
                            </td>
                            <td className="p-2">
                                <button
                                    className="text-red-500 "
                                    onClick={() => handleDeleteLaunchArg(arg.id)}
                                >
                                    Delete
                                </button>
                            </td>
                        </tr>
                    ))}
                </tbody>
            </table>

            <h2 className="mt-8 mb-2">Python Scripts</h2>
            <table className="border-collapse">
                <thead>
                    <tr>
                        <th className="p-2">Script Path</th>
                        <th className="p-2">Created</th>
                        <th className="p-2">Modified</th>
                        <th className="p-2">Accessed</th>
                        <th className="p-2">Actions</th>
                    </tr>
                </thead>
                <tbody>
                    {pythonScripts.map((script) => (
                        <tr key={script.id}>
                            <td className="p-2">{script.script_file_path}</td>
                            <td className="p-2">{script.created}</td>
                            <td className="p-2">{script.modified}</td>
                            <td className="p-2">{script.accessed}</td>
                            <td className="p-2 ">
                                <button
                                    className="text-red-500"
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
