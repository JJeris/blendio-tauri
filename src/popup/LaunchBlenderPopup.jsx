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
        loadPythonScripts();
        loadLaunchArgs();
    }, []);

    const closeWindow = async () => {
        const appWindow = getCurrentWindow();
        await appWindow.close();
    };

    const loadPythonScripts = async () => {
        try {
            const scripts = await invoke("fetch_python_scripts", {
                id: null,
                limit: 20,
                scriptFilePath: null
            });
            setRecentPythonScripts(scripts);
        } catch (err) {
            setRecentPythonScripts([]);
            console.error("Failed to fetch python scripts:", err);
        }
    };

    const loadLaunchArgs = async () => {
        try {
            const args = await invoke("fetch_launch_arguments", {
                id: null,
                limit: 20,
                argumentString: null
            });
            setRecentLaunchArgs(args);
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

    const handleLaunch = async () => {
        try {
            await emit("launch-blender-instance-requested", {
                pythonScriptId: selectedPythonScript?.id || null,
                launchArgs: launchArgs.trim(),
            });
            await closeWindow();
        } catch (err) {
            console.error("Failed to launch blender version:", err);
            await closeWindow();
        }
    };

    return (
        <div className="p-4 ">
            <h2 className="mb-2">Launch Blender Instance</h2>

            <label className="mb-2">Launch Arguments</label>
            <input
                type="text"
                className="mb-4"
                value={launchArgs}
                onChange={(e) => setLaunchArgs(e.target.value)}
                placeholder="e.g. --background --python"
            />


            {recentLaunchArgs.length > 0 && (
                <button
                    className="mb-4   "
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
                    <p >Recent Launch Args:</p>
                    <ul >
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

            {launchArgs && (
                <button
                    onClick={() => setLaunchArgs("")}
                    className="mb-2 text-red-500 "
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
                        className="mt-2 text-red-500"
                    >
                        Clear selected Python script
                    </button>
                )}
            </div>

            <div>
                <button
                    onClick={closeWindow}
                >
                    Cancel
                </button>
                <button
                    onClick={handleLaunch}
                    disabled={needsPythonFile && !selectedPythonScript}
                >
                    Launch
                </button>
            </div>
        </div>
    );
};

export default LaunchBlenderPopup;
