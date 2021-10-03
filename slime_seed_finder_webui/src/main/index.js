const { app, BrowserWindow, ipcMain } = require("electron");
const path = require("path");

function createWindow() {
    const win = new BrowserWindow({
        width: 800,
        height: 600,
        webPreferences: {
            nodeIntegration: false,
            nodeIntegrationInWorker: false,
            contextIsolation: true,
            preload: path.join(__dirname, "preload.js"),
        },
    });

    win.loadFile("src/renderer/index.html");
}

app.whenReady().then(() => {
    app.setAppLogsPath();
    console.info("Log folder set to", app.getPath("logs"));
    let rust_addon = require("../../rust-dist");
    console.log("Loaded rust addon in main.js:", rust_addon);

    createWindow();

    app.on("activate", function() {
        if (BrowserWindow.getAllWindows().length === 0) createWindow();
    });
});

app.on("window-all-closed", function() {
    if (process.platform !== "darwin") app.quit();
});

ipcMain.on("getLogsPath", function(event, options) {
    let path = app.getPath("logs");
    event.returnValue = path;
});
