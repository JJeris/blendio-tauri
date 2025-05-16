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
                } catch (err) {
                    await loadProjectFiles();
                    console.error("Failed to create new project file from popup:", err);
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
                } catch (err) {
                    console.error("Failed to open project file from popup:", err);
                } finally {
                    await loadProjectFiles();
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
            const files = await invoke("fetch_blend_files", {
                id: null,
                limit: null,
                filePath: null,
            });
            setProjectFiles(files);
        } catch (err) {
            setProjectFiles([]);
            console.error("Failed to load .blend project files:", err);
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
        } catch (err) {
            await loadProjectFiles();
            console.error("Failed to open .blend file:", err);
        }
    };

    const handleDelete = async (id) => {
        try {
            await invoke("delete_blend_file", { id });
            await loadProjectFiles();
        } catch (err) {
            await loadProjectFiles();
            console.error("Failed to delete .blend file:", err);
        }
    };

    const handleRevealInExplorer = async (id) => {
        try {
            await invoke("reveal_project_file_in_local_file_system", { id });
        } catch (err) {
            await loadProjectFiles();
            console.error("Failed to reveal .blend file:", err);
        }
    };

    const handleInsertIntoArchive = async (id) => {
        try {
            await invoke("create_project_file_archive_file", { id });
        } catch (err) {
            await loadProjectFiles();
            console.error("Failed to insert .blend file into archive:", err);
        }
    };

    const handleCreateNewBlendFile = async () => {
        try {
            await invoke("instance_popup_window", {
                label: "create-new-project-file-popup",
                title: "Create New Project File",
                urlPath: "popup/CreateBlendPopup",
            });
        } catch (err) {
            await loadProjectFiles();
            console.error("Failed to open popup for creating new .blend file:", err);
        }
    };

    return (
        <div className="p-4">
            <div className="mb-4">
                <h1 >Project Files</h1>
                <button
                    className="mt-2 bg-green-500"
                    onClick={handleCreateNewBlendFile}
                >
                    Create New .blend File
                </button>
            </div>

            <table className="border-collapse">
                <thead>
                    <tr>
                        <th className="p-2">File Name</th>
                        <th className="p-2">File Path</th>
                        <th className="p-2">Associated Blender Series</th>
                        <th className="p-2">Created</th>
                        <th className="p-2">Modified</th>
                        <th className="p-2">Accessed</th>
                        <th className="p-2">Actions</th>
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
                                <td className="p-2">{entry.file_name}</td>
                                <td className="p-2">{entry.file_path}</td>
                                <td className="p-2">
                                    {seriesList.length > 0 ? seriesList.join(", ") : "—"}
                                </td>
                                <td className="p-2">{entry.created}</td>
                                <td className="p-2">{entry.modified}</td>
                                <td className="p-2">{entry.accessed}</td>
                                <td className="p-2">
                                    <button
                                        onClick={() => handleOpen(entry.id)}
                                    >
                                        Open
                                    </button>
                                    <button
                                        className="text-red-500"
                                        onClick={() => handleDelete(entry.id)}
                                    >
                                        Delete
                                    </button>
                                    <button
                                        onClick={() => handleRevealInExplorer(entry.id)}
                                    >
                                        Reveal
                                    </button>
                                    <button
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
                            <td colSpan="8" className="p-4">
                                No project files found.
                            </td>
                        </tr>
                    )}
                </tbody>
            </table>
        </div>
    );
}
