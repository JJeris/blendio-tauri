import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { emit } from "@tauri-apps/api/event";

const LaunchBlenderPopup = () => {
    const [launchArgs, setLaunchArgs] = useState("");
    const [pythonFilePath, setPythonFilePath] = useState("");
    const [selectedPythonScript, setSelectedPythonScript] = useState(null);
    const [recentPythonScripts, setRecentPythonScripts] = useState([]);
    const [recentLaunchArgs, setRecentLaunchArgs] = useState([]);

    const needsPythonFile = /(?:^|\s)(--python|-P)\s*$/.test(launchArgs.trim());

    useEffect(() => {
        fetchPythonScripts();
        fetchLaunchArgs();
    }, []);

    const closeWindow = async () => {
        const appWindow = getCurrentWindow();
        await appWindow.close();
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

    const handleLaunch = async () => {
        await emit("launch-blender-instance-requested", {
            pythonScriptId: selectedPythonScript?.id || null,
            launchArgs: launchArgs.trim(),
        });
        await closeWindow();
    };

    return (
        <div className="p-4 text-sm">
            <h2 className="text-lg font-bold mb-2">Launch Blender Instance</h2>

            <label className="block mb-2">Launch Arguments</label>
            <input
                type="text"
                className="w-full px-2 py-1 border rounded mb-4"
                value={launchArgs}
                onChange={(e) => setLaunchArgs(e.target.value)}
                placeholder="e.g. --background --python"
            />


            {recentLaunchArgs.length > 0 && (
                <button
                    className="mb-4 text-xs text-green-600 hover:underline"
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
                    <p className="text-xs mb-1 text-gray-600">Recent Launch Args:</p>
                    <ul className="space-y-1">
                        {recentLaunchArgs.map((arg) => (
                            <li key={arg.id}>
                                <button
                                    onClick={() => setLaunchArgs(arg.argument_string)}
                                    className="text-left text-xs text-blue-600 hover:underline break-all"
                                >
                                    {arg.argument_string}
                                </button>
                            </li>
                        ))}
                    </ul>
                </div>
            )}

            {launchArgs && (
                <button
                    onClick={() => setLaunchArgs("")}
                    className="mb-2 text-xs text-red-500 hover:underline"
                >
                    Clear launch arguments
                </button>
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
                                            setSelectedPythonScript(script);
                                            setPythonFilePath(script.script_file_path);
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

            <div className="flex justify-end space-x-2">
                <button
                    onClick={closeWindow}
                    className="px-4 py-2 border rounded hover:bg-gray-100"
                >
                    Cancel
                </button>
                <button
                    onClick={handleLaunch}
                    disabled={needsPythonFile && !selectedPythonScript}
                    className={`px-4 py-2 rounded text-white ${!needsPythonFile || selectedPythonScript
                        ? "bg-blue-500"
                        : "bg-gray-400 cursor-not-allowed"
                        }`}
                >
                    Launch
                </button>
            </div>
        </div>
    );
};

export default LaunchBlenderPopup;
