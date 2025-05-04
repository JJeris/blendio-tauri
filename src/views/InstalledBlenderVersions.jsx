import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export default function InstalledBlenderVersions() {
    const [installedBlenderVersions, setInstalledBlenderVersions] = useState([]);

    useEffect(() => {
        loadInstalledBlenderVersions();
    }, []);

    const loadInstalledBlenderVersions = async () => {
        try {
            const installed = await invoke("fetch_installed_blender_versions", {
                id: null,
                limit: null,
                installedBlenderVersions: null
            });
            setInstalledBlenderVersions(installed);
        } catch (e) {
            console.error("Failed to load installed Blender versions:", e);
        }
    };

    const handleSetDefault = async (selectedId) => {
        try {
            const selected = installedBlenderVersions.find((e) => e.id === selectedId);
            await invoke("update_installed_blender_version", {
                id: selectedId,
                isDefault: selected.is_default,
            });
            await loadInstalledBlenderVersions();
        } catch (e) {
            console.error("Failed to set default Blender version:", e);
        }
    };

    const handleDelete = async (id) => {
        try {
            await invoke("uninstall_and_delete_installed_blender_version_data", {
                id: id,
            });
            await loadInstalledBlenderVersions();
        } catch (e) {
            console.error("Failed to set default Blender version:", e);
        }
    };

    const handleLaunch = async (id) => {
        try {
            await invoke("launch_blender_version_with_launch_args", {
                id: id,
                launchArgs: null,
                projectFileId: null,
            });
            await loadInstalledBlenderVersions();
        } catch (e) {
            console.error("Failed to set default Blender version:", e);
        }
    };

    return (
        <div className="p-4">
            <h1 className="text-2xl font-bold mb-4">Installed Blender Versions</h1>
            <table className="w-full border-collapse border text-sm">
                <thead>
                    <tr>
                        <th className="border p-2">Version</th>
                        <th className="border p-2">Variant</th>
                        <th className="border p-2">Install Path</th>
                        <th className="border p-2">Executable</th>
                        <th className="border p-2">Created</th>
                        <th className="border p-2">Modified</th>
                        <th className="border p-2">Accessed</th>
                        <th className="border p-2">Default</th>
                        <th className="border p-2">Actions</th>
                    </tr>
                </thead>
                <tbody>
                    {installedBlenderVersions.map((entry) => (
                        <tr key={entry.id}>
                            <td className="border p-2">{entry.version}</td>
                            <td className="border p-2">{entry.variant_type}</td>
                            <td className="border p-2">{entry.installation_directory_path}</td>
                            <td className="border p-2">{entry.executable_file_path}</td>
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
                            <td className="border p-2 text-center space-x-2">
                                <button
                                    className="text-blue-600 hover:underline"
                                    onClick={() => handleLaunch(entry.id)}
                                >
                                    Launch
                                </button>
                                <button
                                    className="text-red-500 hover:underline"
                                    onClick={() => handleDelete(entry.id)}
                                >
                                    Delete
                                </button>
                            </td>
                        </tr>
                    ))}
                    {installedBlenderVersions.length === 0 && (
                        <tr>
                            <td colSpan="9" className="text-center p-4">
                                No installed versions found.
                            </td>
                        </tr>
                    )}
                </tbody>
            </table>
        </div>
    );
}
