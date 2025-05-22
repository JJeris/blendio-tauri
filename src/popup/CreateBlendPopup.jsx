import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { emit } from "@tauri-apps/api/event";

const CreateBlendPopup = () => {
    const [installedBlenderVersions, setInstalledBlenderVersions] = useState([]);
    const [fileName, setFileName] = useState("");
    const [selectedVersionId, setSelectedVersionId] = useState(null);

    useEffect(() => {
        loadInstalledBlenderVersions();
    }, []);

    const closeWindow = async () => {
        const appWindow = getCurrentWindow();
        await appWindow.close();
    };

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

    const handleCreate = async () => {
        if (!fileName || !selectedVersionId) return;
        await emit("create-project-file-confirmed", {
            fileName,
            versionId: selectedVersionId,
        });
        await closeWindow();
    };

    return (
        <div className="p-4">
            <h2 className="mb-2">Create New .blend File</h2>
            <label className="mb-2">File Name</label>
            <input
                type="text"
                className="mb-4"
                placeholder="Enter file name"
                value={fileName}
                onChange={(e) => setFileName(e.target.value)}
            />
            <br />
            <label className="mb-2">Select Blender Version</label>
            <ul className="mb-4">
                {installedBlenderVersions.map((v) => (
                    <li key={v.id}>
                        <button
                            onClick={() => {
                                setSelectedVersionId(v.id)
                            }
                            }
                        >
                            {v.version} {v.variant_type}
                        </button>
                    </li>
                ))}
            </ul>

            <div>
                <button
                    onClick={closeWindow}
                >
                    Cancel
                </button>
                <button
                    onClick={handleCreate}
                    disabled={!fileName || !selectedVersionId}
                >
                    Create
                </button>
            </div>
        </div>
    );
};

export default CreateBlendPopup;
