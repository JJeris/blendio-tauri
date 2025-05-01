import { download } from "@tauri-apps/plugin-upload";

export async function downloadFile(url, filePath, buttonId) {
    const button = document.getElementById(buttonId);
    if (!button) return;

    let accumulated = 0;
    const originalText = button.textContent;
    button.disabled = true;
    button.textContent = "Starting...";
    try {
        await download(url, filePath, ({ progress, total }) => {
            accumulated += progress;
            const percent = Math.floor((accumulated / total) * 100);
            const button = document.getElementById(buttonId);
            if (button)  {
                button.textContent = `Downloading... ${percent}%`;
                button.disabled = true;
            }
        });

        const finishedButton = document.getElementById(buttonId);
        if (finishedButton) {
            finishedButton.textContent = originalText;
            finishedButton.disabled = false;
        } 
    } catch (e) {
        console.error("Download failed:", e);
        if (button) button.textContent = "Error";
    } finally {
        if (button) button.disabled = false;
    }
}
