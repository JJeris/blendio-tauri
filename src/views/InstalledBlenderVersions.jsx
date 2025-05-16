import React, { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export default function InstalledBlenderVersions() {
    const [installedBlenderVersions, setInstalledBlenderVersions] = useState([]);
    const pendingLaunchVersionRef = useRef(null);

    useEffect(() => {
        loadInstalledBlenderVersions();

        const unlisten = listen("launch-blender-instance-requested", async (event) => {
            const { pythonScriptId, launchArgs } = event.payload;
            const versionId = pendingLaunchVersionRef.current;
            if (!versionId) {
                console.error("Missing versionId — did you forget to set the ref?");
                return;
            }

            let launchArgumentId = null;
            try {
                // Only insert launchArgs if it's a non-empty string.
                if (launchArgs && launchArgs.trim() !== "") {
                    launchArgumentId = await invoke("insert_launch_argument", {
                        argumentString: launchArgs.trim(),
                        projectFileId: null,
                        pythonScriptId: pythonScriptId || null,
                    });
                }
                await invoke("launch_blender_version_with_launch_args", {
                    id: versionId,
                    launchArgumentsId: launchArgumentId || null,
                    pythonScriptId: pythonScriptId || null,
                });
                await loadInstalledBlenderVersions();
            } catch (err) {
                console.error("Failed to launch Blender version from popup:", err);
            } finally {
                pendingLaunchVersionRef.current = null;
            }
        });

        return () => {
            unlisten.then((f) => f());
        };
    }, []);

    const loadInstalledBlenderVersions = async () => {
        try {
            await invoke("insert_and_refresh_installed_blender_versions");
            const versions = await invoke("fetch_installed_blender_versions", {
                id: null,
                limit: null,
                executableFilePath: null
            });
            setInstalledBlenderVersions(versions);
        } catch (err) {
            setInstalledBlenderVersions([]);
            console.error("Failed to load installed Blender versions:", err);
        }
    };

    const handleSetDefault = async (selectedId) => {
        try {
            await invoke("update_installed_blender_version", {
                id: selectedId,
                isDefault: installedBlenderVersions.find((e) => e.id === selectedId).is_default
            });
            await loadInstalledBlenderVersions();
        } catch (err) {
            await loadInstalledBlenderVersions();
            console.error("Failed to set default Blender version:", err);
        }
    };

    const handleDelete = async (selectedId) => {
        try {
            await invoke("uninstall_and_delete_installed_blender_version_data", {
                id: selectedId,
            });
            await loadInstalledBlenderVersions();
        } catch (err) {
            await loadInstalledBlenderVersions();
            console.error("Failed to delete Blender version:", err);
        }
    };

    const handleLaunch = async (id) => {
        pendingLaunchVersionRef.current = id;
        try {
            await invoke("instance_popup_window", {
                label: "launch-blender-version-popup",
                title: "Launch Blender Version",
                urlPath: "popup/LaunchBlenderPopup"
            });
        } catch (err) {
            await loadInstalledBlenderVersions();
            console.error("Failed to open launch popup:", err);
        }
    };

    return (
        <div className="p-4">
            <h1 className="mb-4">Installed Blender Versions</h1>
            <table className="border-collapse">
                <thead>
                    <tr>
                        <th className="p-2">Version</th>
                        <th className="p-2">Variant</th>
                        <th className="p-2">Installation Path</th>
                        <th className="p-2">Executable file path</th>
                        <th className="p-2">Created</th>
                        <th className="p-2">Modified</th>
                        <th className="p-2">Accessed</th>
                        <th className="p-2">Default</th>
                        <th className="p-2">Actions</th>
                    </tr>
                </thead>
                <tbody>
                    {installedBlenderVersions.map((entry) => (
                        <tr key={entry.id}>
                            <td className="p-2">{entry.version}</td>
                            <td className="p-2">{entry.variant_type}</td>
                            <td className="p-2">{entry.installation_directory_path}</td>
                            <td className="p-2">{entry.executable_file_path}</td>
                            <td className="p-2">{entry.created}</td>
                            <td className="p-2">{entry.modified}</td>
                            <td className="p-2">{entry.accessed}</td>
                            <td className="p-2 ">
                                <input
                                    type="checkbox"
                                    checked={entry.is_default}
                                    onChange={() => handleSetDefault(entry.id)}
                                />
                            </td>
                            <td className="p-2">
                                <button
                                    onClick={() => handleLaunch(entry.id)}
                                >
                                    Launch
                                </button>
                                <button
                                    className="text-red-500"
                                    onClick={() => handleDelete(entry.id)}
                                >
                                    Delete
                                </button>
                            </td>
                        </tr>
                    ))}
                    {installedBlenderVersions.length === 0 && (
                        <tr>
                            <td colSpan="9" className="p-4">
                                No installed versions found.
                            </td>
                        </tr>
                    )}
                </tbody>
            </table>
        </div>
    );
}
