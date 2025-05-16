import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { emit } from "@tauri-apps/api/event";

const LaunchBlendPopup = () => {
    const [installedBlenderVersions, setInstalledBlenderVersions] = useState([]);
    const [selectedVersionId, setSelectedVersionId] = useState(null);
    const [launchArgs, setLaunchArgs] = useState("");
    const [pythonFilePath, setPythonFilePath] = useState("");
    const [selectedPythonScript, setSelectedPythonScript] = useState(null);
    const [recentPythonScripts, setRecentPythonScripts] = useState([]);
    const [recentLaunchArgs, setRecentLaunchArgs] = useState([]);

    // Match if last argument is --python or -P (trailing whitespace allowed)
    const needsPythonFile = /(?:^|\s)(--python|-P)\s*$/.test(launchArgs.trim());

    useEffect(() => {
        loadInstalledBlenderVersions();
        loadPythonScripts();
        loadLaunchArgs();
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

    const loadPythonScripts = async () => {
        try {
            const result = await invoke("fetch_python_scripts", {
                id: null,
                limit: 20,
                scriptFilePath: null
            });
            setRecentPythonScripts(result);
        } catch (err) {
            setRecentPythonScripts([]);
            console.error("Failed to fetch recent python scripts:", err);
        }
    };
    const loadLaunchArgs = async () => {
        try {
            const result = await invoke("fetch_launch_arguments", {
                id: null,
                limit: 20,
                argumentString: null
            });
            setRecentLaunchArgs(result);
        } catch (err) {
            setRecentLaunchArgs([]);
            console.error("Failed to fetch launch arguments:", err);
        }
    };

    const handlePythonFileSelect = async () => {
        try {
            const pythonScript = await invoke("insert_python_script");
            if (pythonScript) {
                setSelectedPythonScript(pythonScript);
                setPythonFilePath(pythonScript.script_file_path);
            }
            loadPythonScripts();
        } catch (err) {
            console.error("Failed to select python file:", err);
        }
    };

    const handleOpen = async () => {
        if (!selectedVersionId) return;
        if (needsPythonFile && !selectedPythonScript) return;
        await emit("open-project-file-confirmed", {
            versionId: selectedVersionId,
            pythonScriptId: selectedPythonScript?.id || null,
            launchArgs: launchArgs.trim(),
        });
        await closeWindow();
    };

    return (
        <div className="p-4">
            <h2 className="mb-2">Open Blend File</h2>

            <label className="mb-2">Launch Arguments</label>
            <input
                type="text"
                className="mb-4"
                placeholder="e.g., --background --python"
                value={launchArgs}
                onChange={(e) => setLaunchArgs(e.target.value)}
            />
            <br />


            {recentLaunchArgs.length > 0 && (
                <button
                    className="mb-4"
                    onClick={() => {
                        const defaultArg = recentLaunchArgs.find(arg => arg.is_default) || recentLaunchArgs[0];
                        if (defaultArg) setLaunchArgs(defaultArg.argument_string);
                    }}
                >
                    Use Default Launch Argument{" "}
                    {recentLaunchArgs.find(arg => arg.is_default)
                        ? `(${recentLaunchArgs.find(arg => arg.is_default).argument_string})`
                        : `(${recentLaunchArgs[0]?.argument_string})`}
                </button>
            )}

            {recentLaunchArgs.length > 0 && (
                <div className="mb-4">
                    <p>Recently Used Launch Args:</p>
                    <ul>
                        {recentLaunchArgs.map((arg) => (
                            <li key={arg.id}>
                                <button
                                    onClick={() => setLaunchArgs(arg.argument_string)}
                                >
                                    {arg.argument_string}
                                </button>
                            </li>
                        ))}
                    </ul>
                </div>
            )}
            <br />
            {launchArgs && (
                <button
                    onClick={() => setLaunchArgs("")}
                    className="mb-2 text-red-500"
                >
                    Clear launch arguments
                </button>
            )}

            <div className="mb-4">
                <button
                    onClick={handlePythonFileSelect}
                    disabled={!needsPythonFile}
                >
                    {pythonFilePath ? "Python File Selected" : "Select Python Script"}
                </button>
                {pythonFilePath && (
                    <p>{pythonFilePath}</p>
                )}


                {recentPythonScripts.length > 0 && (
                    <div className="mt-2">
                        <p>Recently Used Scripts:</p>
                        <ul>
                            {recentPythonScripts.map((script) => (
                                <li key={script.id}>
                                    <button
                                        disabled={!needsPythonFile}
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
                        className="mt-2 text-red-500"
                    >
                        Clear selected Python script
                    </button>
                )}
            </div>

            <br />
            {installedBlenderVersions.length > 0 && (
                <button
                    className="mb-4"
                    onClick={() => {
                        const defaultVersion = installedBlenderVersions.find((v) => v.is_default === true) || installedBlenderVersions[0];
                        if (defaultVersion) setSelectedVersionId(defaultVersion.id);
                    }}
                >
                    Use Default Version{" "}
                    {installedBlenderVersions.find((v) => v.is_default)?.version
                        ? installedBlenderVersions.find((v) => v.is_default).version + " " + installedBlenderVersions.find((v) => v.is_default).variant_type
                        : installedBlenderVersions[0]?.version + " " + installedBlenderVersions[0]?.variant_type}
                </button>
            )}
            <br />
            <label className="mb-2">Select Blender Version</label>
            <ul className="mb-4">
                {installedBlenderVersions.map((v) => (
                    <li key={v.id}>
                        <button
                            onClick={() => setSelectedVersionId(v.id)}
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
                    onClick={handleOpen}
                    disabled={
                        !selectedVersionId || (needsPythonFile && !selectedPythonScript)
                    }
                >
                    Open
                </button>
            </div>
        </div>
    );
};

export default LaunchBlendPopup;
