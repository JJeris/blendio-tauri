import React, { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export default function ProjectFiles() {
    const [projectFiles, setProjectFiles] = useState([]);
    const pendingOpenProjectRef = useRef(null);

    useEffect(() => {
        loadProjectFiles();

        const unlistenPromise = Promise.all([
            listen("create-project-file-confirmed", async (event) => {
                const { fileName, versionId } = event.payload;
                try {
                    await invoke("create_new_project_file", {
                        installedBlenderVersionId: versionId,
                        fileName,
                    });
                    await loadProjectFiles();
                } catch (e) {
                    console.error("Failed to create new project file from popup:", e);
                }
            }),
            listen("open-project-file-confirmed", async (event) => {
                const { versionId, pythonScriptId, launchArgs } = event.payload;
                const projectFileId = pendingOpenProjectRef.current;
                if (!projectFileId) {
                    console.error("Missing projectFileId — did you forget to set the ref?");
                    return;

                }

                let launchArgumentId = null;
                try {
                    // Only insert launchArgs if it's a non-empty string.
                    if (launchArgs && launchArgs.trim() !== "") {
                        launchArgumentId = await invoke("insert_launch_argument", {
                            argumentString: launchArgs.trim(),
                            projectFileId,
                            pythonScriptId: pythonScriptId || null,
                        });
                    }
                    await invoke("open_blend_file", {
                        id: projectFileId,
                        installedBlenderVersionId: versionId,
                        pythonScriptId: pythonScriptId || null,
                        launchArgumentsId: launchArgumentId || null,
                    });
                    loadProjectFiles();
                } catch (error) {
                    console.error("Failed to open project file from popup:", e);
                } finally {
                    pendingOpenProjectRef.current = null;
                }
            }),
        ]);

        return () => {
            unlistenPromise.then((unlisteners) =>
                unlisteners.forEach((unlisten) => unlisten())
            );
        };
    }, []);


    const loadProjectFiles = async () => {
        try {
            await invoke("insert_and_refresh_blend_files");
            const result = await invoke("fetch_blend_files", {
                id: null,
                limit: null,
                filePath: null,
            });
            setProjectFiles(result);
        } catch (e) {
            console.error("Failed to load .blend project files:", e);
        }
    };

    const handleOpen = async (id) => {
        pendingOpenProjectRef.current = id;
        try {
            await invoke("instance_popup_window", {
                label: "launch-project-file-popup",
                title: "Launch Project File",
                urlPath: "popup/LaunchBlendPopup",
            });
        } catch (e) {
            console.error("Failed to open .blend file:", e);
        }
    };

    const handleDelete = async (id) => {
        try {
            await invoke("delete_blend_file", { id });
            await loadProjectFiles();
        } catch (e) {
            console.error("Failed to delete .blend file:", e);
        }
    };

    const handleRevealInExplorer = async (id) => {
        try {
            await invoke("reveal_project_file_in_local_file_system", { id });
        } catch (e) {
            console.error("Failed to reveal .blend file:", e);
        }
    };

    const handleInsertIntoArchive = async (id) => {
        try {
            await invoke("create_project_file_archive_file", { id });
        } catch (e) {
            console.error("Failed to insert .blend file into archive:", e);
        }
    };

    const handleCreateNewBlendFile = async () => {
        try {
            await invoke("instance_popup_window", {
                label: "create-new-project-file-popup",
                title: "Create New Project File",
                urlPath: "popup/CreateBlendPopup",
            });
        } catch (e) {
            console.error("Failed to open popup for creating new .blend file:", e);
        }
    };

    const handleInsertLaunchArgument = async (argumentString, projectFileId, pythonScriptId) => {
        try {
            await invoke("insert_launch_argument", {
                argumentString,
                projectFileId,
                pythonScriptId
            });
        } catch (e) {
            console.error("Failed to insert launch argument into the db", e);
        }
    }

    return (
        <div className="p-4">
            <div className="flex justify-between items-center mb-4">
                <h1 className="text-2xl font-bold">.blend Project Files</h1>
                <button
                    className="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700"
                    onClick={handleCreateNewBlendFile}
                >
                    Create New .blend File
                </button>
            </div>

            <table className="w-full border-collapse border text-sm">
                <thead>
                    <tr>
                        <th className="border p-2">File Name</th>
                        <th className="border p-2">File Path</th>
                        <th className="border p-2">Series</th>
                        <th className="border p-2">Last Blender Version</th>
                        <th className="border p-2">Created</th>
                        <th className="border p-2">Modified</th>
                        <th className="border p-2">Accessed</th>
                        <th className="border p-2">Actions</th>
                    </tr>
                </thead>
                <tbody>
                    {projectFiles.map((entry) => {
                        let seriesList = [];

                        try {
                            seriesList = JSON.parse(entry.associated_series_json);
                        } catch {
                            seriesList = [];
                        }

                        return (
                            <tr key={entry.id}>
                                <td className="border p-2">{entry.file_name}</td>
                                <td className="border p-2">{entry.file_path}</td>
                                <td className="border p-2">
                                    {seriesList.length > 0 ? seriesList.join(", ") : "—"}
                                </td>
                                <td className="border p-2">
                                    {entry.last_used_blender_version_id ?? "N/A"}
                                </td>
                                <td className="border p-2">{entry.created}</td>
                                <td className="border p-2">{entry.modified}</td>
                                <td className="border p-2">{entry.accessed}</td>
                                <td className="border p-2 space-x-2 text-center">
                                    <button
                                        className="text-blue-600 hover:underline"
                                        onClick={() => handleOpen(entry.id)}
                                    >
                                        Open
                                    </button>
                                    <button
                                        className="text-red-500 hover:underline"
                                        onClick={() => handleDelete(entry.id)}
                                    >
                                        Delete
                                    </button>
                                    <button
                                        className="text-gray-600 hover:underline"
                                        onClick={() => handleRevealInExplorer(entry.id)}
                                    >
                                        Reveal
                                    </button>
                                    <button
                                        className="text-purple-600 hover:underline"
                                        onClick={() => handleInsertIntoArchive(entry.id)}
                                    >
                                        Archive
                                    </button>
                                </td>
                            </tr>
                        );
                    })}
                    {projectFiles.length === 0 && (
                        <tr>
                            <td colSpan="8" className="text-center p-4">
                                No project files found.
                            </td>
                        </tr>
                    )}
                </tbody>
            </table>
        </div>
    );
}
