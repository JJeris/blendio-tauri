// src/views/BlenderDownload.jsx
import React, { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { downloadFile } from "../utils/web";
import { listen } from "@tauri-apps/api/event";

export default function BlenderDownload() {
    const [downloadableBuilds, setDownloadableBuilds] = useState([]);
    const pendingDownloadRef = useRef(null);

    useEffect(() => {
        loadBlenderBuilds();

        const unlisten = listen("download-path-selected", async (event) => {
            const selectedPath = event.payload?.path;
            const pending = pendingDownloadRef.current;
            if (selectedPath && pending) {
                const { build, url, fileName, buttonId } = pending;
                pendingDownloadRef.current = null;
                await downloadFile(url, `${selectedPath}\\${fileName}`, buttonId);
                await invoke("download_and_install_blender_version", {
                    archiveFilePath: `${selectedPath}\\${fileName}`,
                    downloadableBlenderVersion: build,
                });
            }
        });

        return () => {
            unlisten.then((off) => off());
        };
    }, []);

    const loadBlenderBuilds = async () => {
        try {
            const builds = await invoke("get_downloadable_blender_version_data");
            setDownloadableBuilds(builds);
        } catch (e) {
            console.error("Failed to fetch downloadable blender versions:", e);
        }
    };

    const handleOpenPopup = async () => {
        try {
            await invoke("instance_popup_window", {
                label: "download-popup",
                title: "Choose Download Location",
                urlPath: "popup/DownloadPopup"
            });
        } catch (e) {
            console.error("Failed to open popup:", e);
        }
    };

    const download = async (build, url, fileName, buttonId) => {
        pendingDownloadRef.current = { build, url, fileName, buttonId };
        try {
            await handleOpenPopup();
        } catch (error) {
            console.error("Failed to open popup:", error);
        }
    };

    return (
        <div className="p-4">
            <h1 className="text-2xl font-bold mb-4">Blender Daily Builds</h1>
            <table className="w-full border-collapse border text-sm">
                <thead>
                    <tr>
                        <th className="border p-2">Version</th>
                        <th className="border p-2">App</th>
                        <th className="border p-2">Risk</th>
                        <th className="border p-2">Branch</th>
                        <th className="border p-2">Platform</th>
                        <th className="border p-2">Arch</th>
                        <th className="border p-2">Bit</th>
                        <th className="border p-2">Extension</th>
                        <th className="border p-2">Size</th>
                        <th className="border p-2">Download</th>
                    </tr>
                </thead>
                <tbody>
                    {downloadableBuilds.map((build, index) => {
                        const buttonId = `download-btn-${build.file_name}`;
                        return (
                            <tr key={index}>
                                <td className="border p-2">{build.version}</td>
                                <td className="border p-2">{build.app}</td>
                                <td className="border p-2">{build.risk_id}</td>
                                <td className="border p-2">{build.branch}</td>
                                <td className="border p-2">{build.platform}</td>
                                <td className="border p-2">{build.architecture}</td>
                                <td className="border p-2">{build.bitness}</td>
                                <td className="border p-2">{build.file_extension}</td>
                                <td className="border p-2">{build.file_size}</td>
                                <td className="border p-2">
                                    <button
                                        id={buttonId}
                                        className="bg-blue-600 text-white px-4 py-2 rounded"
                                        onClick={() => download(build, build.url, build.file_name, buttonId)}
                                    >
                                        Download
                                    </button>
                                </td>
                            </tr>
                        );
                    })}
                    {downloadableBuilds.length === 0 && (
                        <tr>
                            <td colSpan="10" className="text-center p-4">No builds found.</td>
                        </tr>
                    )}
                </tbody>
            </table>
        </div>
    );
}
