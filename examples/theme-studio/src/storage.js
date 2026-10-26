const key = "rustify-ui.theme-studio.v2";
const legacyKey = "rustify-ui.theme-studio.v1";
const backupPrefix = "rustify-ui.theme-studio.v1.backup.";

export function makeStorageGuard() { return new AbortController(); }
export function storageAbortHandle(guard) { return guard.abort.bind(guard); }
export function storageAlive(guard) { return !guard.signal.aborted; }
export function storageWritable() {
    try { return globalThis.isSecureContext === true && typeof navigator.locks?.request === "function" && !!globalThis.localStorage; }
    catch { return false; }
}
function alive(guard) {
    if (guard.signal.aborted) throw new DOMException("Storage owner was disposed", "AbortError");
}
function backups(storage) {
    const values = {};
    const keys = [];
    for (let i = 0; i < storage.length; i++) {
        const name = storage.key(i);
        if (name?.startsWith(backupPrefix)) keys.push(name);
    }
    for (const name of keys.sort()) values[name] = storage.getItem(name);
    return values;
}
export function storageBackups() { return JSON.stringify(backups(localStorage)); }

export async function commitStorage(raw, guard) {
    if (!storageWritable()) throw new Error("Web Locks or secure local storage unavailable; memory editing and downloads remain available");
    const request = JSON.parse(raw);
    alive(guard);
    return navigator.locks.request(request.lock_name, {signal: guard.signal}, () => {
        alive(guard);
        const storage = localStorage;
        const current = storage.getItem(key);
        alive(guard);
        if (current !== request.expected) return JSON.stringify({kind:"conflict",raw:current});
        if (request.kind === "clear-backups") {
            if (JSON.stringify(backups(storage)) !== JSON.stringify(request.backups)) return JSON.stringify({kind:"backup-conflict"});
            alive(guard);
            for (const name of Object.keys(request.backups)) { alive(guard); storage.removeItem(name); }
            if (Object.keys(request.backups).some(name => storage.getItem(name) !== null)) throw new Error("Backup removal read-back failed; review the retained backups before retrying");
            return JSON.stringify({kind:"cleared-backups"});
        }
        if (request.kind === "load") return JSON.stringify({kind:"loaded",raw:current});
        if (request.kind === "clear") {
            alive(guard);
            storage.removeItem(key);
            if (storage.getItem(key) !== null) return JSON.stringify({kind:"conflict",raw:storage.getItem(key)});
            return JSON.stringify({kind:"cleared"});
        }
        if (request.legacy !== null) {
            if (storage.getItem(legacyKey) !== request.legacy) return JSON.stringify({kind:"legacy-changed"});
            const backup = request.backup;
            const existing = storage.getItem(backup.key);
            if (existing !== null && existing !== backup.raw) throw new Error("Backup key exists with different bytes; no data was overwritten");
            alive(guard);
            if (existing === null) storage.setItem(backup.key, backup.raw);
            if (storage.getItem(backup.key) !== backup.raw) throw new Error("Backup read-back failed; migration was not committed");
            if (storage.getItem(legacyKey) !== request.legacy) return JSON.stringify({kind:"legacy-changed"});
            if (storage.getItem(key) !== request.expected) return JSON.stringify({kind:"conflict",raw:storage.getItem(key)});
        }
        alive(guard);
        storage.setItem(key, request.write);
        if (storage.getItem(key) !== request.write) throw new Error("Storage read-back did not match the committed snapshot");
        return JSON.stringify({kind:"written",raw:request.write});
    });
}
