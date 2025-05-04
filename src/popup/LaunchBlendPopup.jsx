import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { emit } from "@tauri-apps/api/event";

const LaunchBlendPopup = () => {
    const [versions, setVersions] = useState([]);
    const [selectedVersionId, setSelectedVersionId] = useState(null);
    const [launchArgs, setLaunchArgs] = useState("");
    const [pythonFilePath, setPythonFilePath] = useState("");
    const [selectedPythonScript, setSelectedPythonScript] = useState(null);
    const [recentPythonScripts, setRecentPythonScripts] = useState([]);
    const [recentLaunchArgs, setRecentLaunchArgs] = useState([]);

    useEffect(() => {
        const fetchVersions = async () => {
            const result = await invoke("fetch_installed_blender_versions", {
                id: null,
                limit: null,
                installedBlenderVersions: null
            });
            setVersions(result);
        };
        const fetchPythonScripts = async () => {
            try {
                const result = await invoke("fetch_python_scripts", {
                    id: null,
                    limit: 20,
                    scriptFilePath: null
                });
                setRecentPythonScripts(result);
            } catch (e) {
                console.error("Failed to fetch recent python scripts:", e);
            }
        };
        const fetchLaunchArgs = async () => {
            try {
                const result = await invoke("fetch_launch_arguments", {
                    id: null,
                    limit: 20,
                    argumentString: null
                });
                setRecentLaunchArgs(result);
            } catch (e) {
                console.error("Failed to fetch recent launch arguments:", e);
            }
        };

        fetchVersions();
        fetchPythonScripts();
        fetchLaunchArgs();
    }, []);

    const closeWindow = async () => {
        const appWindow = getCurrentWindow();
        await appWindow.close();
    };

    const handlePythonFileSelect = async () => {
        try {
            const pythonScript = await invoke("insert_python_script");
            setSelectedPythonScript(pythonScript);
            setPythonFilePath(pythonScript.script_file_path);

            const updated = await invoke("fetch_python_scripts", {
                id: null,
                limit: 20,
                scriptFilePath: null
            });
            setRecentPythonScripts(updated);
        } catch (e) {
            console.error("Failed to select python file:", e);
        }
    };

    const handleOpen = async () => {
        if (!selectedVersionId) return;
        if (needsPythonFile && !selectedPythonScript) return;
        console.log(selectedVersionId)
        console.log(selectedPythonScript)
        console.log(launchArgs)
        await emit("open-project-file-confirmed", {
            versionId: selectedVersionId,
            pythonScriptId: selectedPythonScript?.id || null,
            launchArgs: launchArgs.trim(),
        });
        await closeWindow();
    };

    // Match if last argument is --python or -P (trailing whitespace allowed)
    const needsPythonFile = /(?:^|\s)(--python|-P)\s*$/.test(launchArgs.trim());

    return (
        <div className="p-4 text-sm">
            <h2 className="text-lg font-bold mb-2">Open Blend File</h2>

            <label className="block mb-2">Launch Arguments</label>
            <input
                type="text"
                className="w-full px-2 py-1 border rounded mb-4"
                placeholder="e.g., --background --python"
                value={launchArgs}
                onChange={(e) => setLaunchArgs(e.target.value)}
            />
            <br />
            {launchArgs && (
                <button
                    onClick={() => setLaunchArgs("")}
                    className="mb-2 text-xs text-red-500 hover:underline"
                >
                    Clear launch arguments
                </button>
            )}

            {recentLaunchArgs.length > 0 && (
                <div className="mb-4">
                    <p className="text-xs mb-1 text-gray-600">Recently Used Launch Args:</p>
                    <ul className="space-y-1">
                        {recentLaunchArgs.map((arg) => (
                            <li key={arg.id}>
                                <button
                                    className="text-left text-xs text-blue-600 hover:underline break-all"
                                    onClick={() => setLaunchArgs(arg.argument_string)}
                                >
                                    {arg.argument_string}
                                </button>
                            </li>
                        ))}
                    </ul>
                </div>
            )}

            <div className="mb-4">
                <button
                    className={`px-3 py-1 border rounded text-sm ${needsPythonFile
                        ? "bg-yellow-100 hover:bg-yellow-200"
                        : "bg-gray-100 text-gray-400 cursor-not-allowed"
                        }`}
                    onClick={handlePythonFileSelect}
                    disabled={!needsPythonFile}
                >
                    {pythonFilePath ? "Python File Selected" : "Select Python Script"}
                </button>
                {pythonFilePath && (
                    <p className="mt-1 text-xs break-all text-gray-600">{pythonFilePath}</p>
                )}


                {recentPythonScripts.length > 0 && (
                    <div className="mt-2">
                        <p className="text-xs mb-1 text-gray-600">Recently Used Scripts:</p>
                        <ul className="space-y-1">
                            {recentPythonScripts.map((script) => (
                                <li key={script.id}>
                                    <button
                                        disabled={!needsPythonFile}
                                        className={`text-left text-xs break-all ${needsPythonFile
                                            ? "text-blue-600 hover:underline"
                                            : "text-gray-400 cursor-not-allowed"
                                            }`}
                                        onClick={() => {
                                            if (needsPythonFile) {
                                                setSelectedPythonScript(script);
                                                setPythonFilePath(script.script_file_path);
                                            }
                                        }}
                                    >
                                        {script.script_file_path}
                                    </button>
                                </li>
                            ))}
                        </ul>
                    </div>
                )}

                {selectedPythonScript && (
                    <button
                        onClick={() => {
                            setSelectedPythonScript(null);
                            setPythonFilePath("");
                        }}
                        className="mt-2 text-xs text-red-500 hover:underline"
                    >
                        Clear selected Python script
                    </button>
                )}
            </div>

            <br />
            {versions.length > 0 && (
                <button
                    className="mb-4 px-4 py-2 bg-blue-500 text-white rounded"
                    onClick={() => {
                        const defaultVersion = versions.find((v) => v.is_default === true) || versions[0];
                        if (defaultVersion) setSelectedVersionId(defaultVersion.id);
                    }}
                >
                    Use Default Version{" "}
                    {versions.find((v) => v.is_default)?.version
                        ? versions.find((v) => v.is_default).version + " " + versions.find((v) => v.is_default).variant_type
                        : versions[0]?.version + " " + versions[0]?.variant_type}
                </button>
            )}
            <br />
            <label className="block mb-2">Select Blender Version</label>
            <ul className="space-y-2 mb-4">
                {versions.map((v) => (
                    <li key={v.id}>
                        <button
                            className={`w-full text-left px-2 py-1 border rounded hover:bg-gray-100 ${selectedVersionId === v.id ? "bg-blue-100 border-blue-400" : ""
                                }`}
                            onClick={() => setSelectedVersionId(v.id)}
                        >
                            {v.version} {v.variant_type}
                        </button>
                    </li>
                ))}
            </ul>

            <div className="flex justify-end space-x-2">
                <button
                    onClick={closeWindow}
                    className="px-4 py-2 border rounded hover:bg-gray-100"
                >
                    Cancel
                </button>
                <button
                    onClick={handleOpen}
                    disabled={
                        !selectedVersionId || (needsPythonFile && !selectedPythonScript)
                    }
                    className={`px-4 py-2 rounded text-white ${selectedVersionId && (!needsPythonFile || selectedPythonScript)
                        ? "bg-blue-500"
                        : "bg-gray-400 cursor-not-allowed"
                        }`}
                >
                    Open
                </button>
            </div>
        </div>
    );
};

export default LaunchBlendPopup;
