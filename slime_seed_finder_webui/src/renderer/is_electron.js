function isElectron() {
    // https://github.com/cheton/is-electron/blob/7ee928893deea5c47d450356b5204907bc0c3215/index.js#L14
    if (
        typeof navigator === "object" &&
        typeof navigator.userAgent === "string" &&
        navigator.userAgent.indexOf("Electron") >= 0
    ) {
        return true;
    }

    return false;
}
