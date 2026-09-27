import type { Game } from "../_game/game";
import { deserialize, isEmpty, serialize } from "../_game/storage";
import { storageKey, type StoredGame } from "../_game/stored";
import { remoteStore } from "./remote";
import type { Timer } from "./timer";

const LOCAL_DELAY_MS = 300;
const REMOTE_DELAY_MS = 2000;

function readLocal(key: string): StoredGame | null {
    try {
        const raw = localStorage.getItem(key);
        return raw ? (JSON.parse(raw) as StoredGame) : null;
    } catch (error) {
        console.warn("[persistence] read failed", error);
        return null;
    }
}

function writeLocal(key: string, value: StoredGame | null): void {
    try {
        if (value) localStorage.setItem(key, JSON.stringify(value));
        else localStorage.removeItem(key);
    } catch (error) {
        console.warn("[persistence] write failed", error);
    }
}

export function attachPersistence(game: Game, puzzleKey: string, timer: Timer): void {
    const key = storageKey(puzzleKey);
    const remote = remoteStore(puzzleKey);

    let current = readLocal(key);
    let remoteVersion = current?.savedAt ?? 0;
    let dirty = false;
    let localPending: ReturnType<typeof setTimeout> | undefined;
    let remotePending: ReturnType<typeof setTimeout> | undefined;

    const restore = (stored: StoredGame): boolean => {
        const state = deserialize(game.puzzle, puzzleKey, stored);
        if (!state) return false;
        timer.setElapsed(stored.elapsedMs);
        game.dispatch({ type: "restore", state });
        dirty = false;
        return true;
    };

    const adopt = (theirs: StoredGame) => {
        if (!restore(theirs)) return;
        current = theirs;
        writeLocal(key, theirs);
    };

    const snapshot = (): StoredGame | null => {
        if (isEmpty(game.state) && timer.elapsed() === 0) return null;
        return serialize(puzzleKey, game.state, timer.elapsed(), current);
    };

    const flushLocal = () => {
        clearTimeout(localPending);
        localPending = undefined;
        current = snapshot();
        writeLocal(key, current);
    };

    const flushRemote = async (keepalive = false, force = false): Promise<void> => {
        clearTimeout(remotePending);
        remotePending = undefined;
        flushLocal();

        const value = current;
        if (!value) {
            if (remoteVersion > 0) {
                remoteVersion = 0;
                await remote.clear(keepalive);
            }
            return;
        }

        const result = await remote.save(value, keepalive);

        if (result.kind === "saved") {
            remoteVersion = result.savedAt;
            dirty = false;
            if (current) {
                current = { ...current, savedAt: result.savedAt, solvedAt: result.solvedAt };
                writeLocal(key, current);
            }
            return;
        }

        if (result.kind === "stale") {
            remoteVersion = result.theirs.savedAt;
            if (!dirty) {
                adopt(result.theirs);
            } else if (!force && current) {
                current = { ...current, savedAt: result.theirs.savedAt };
                writeLocal(key, current);
                await flushRemote(keepalive, true);
            }
        }
    };

    const flushAll = (keepalive = false) => {
        flushLocal();
        void flushRemote(keepalive);
    };

    const scheduleLocal = () => {
        clearTimeout(localPending);
        localPending = setTimeout(flushLocal, LOCAL_DELAY_MS);
    };

    const schedule = () => {
        dirty = true;
        scheduleLocal();
        if (!remotePending) remotePending = setTimeout(() => void flushRemote(), REMOTE_DELAY_MS);
    };

    game.on("entry", schedule);
    game.on("check", schedule);
    game.on("cursor", scheduleLocal);
    game.on("completion", () => flushAll());
    game.on("reset", () => flushAll());

    document.addEventListener("visibilitychange", () => {
        if (document.visibilityState === "hidden") flushAll(true);
    });
    window.addEventListener("pagehide", () => flushAll(true));

    if (current && !restore(current)) {
        current = null;
        remoteVersion = 0;
        writeLocal(key, null);
    }

    void remote.load().then((theirs) => {
        if (!theirs) {
            if (!current) return;
            if (current.savedAt > 0) {
                current = null;
                remoteVersion = 0;
                writeLocal(key, null);
                game.dispatch({ type: "reset" });
                return;
            }
            void flushRemote();
            return;
        }

        remoteVersion = theirs.savedAt;
        if (!current || theirs.savedAt > current.savedAt) {
            adopt(theirs);
        } else if (theirs.savedAt < current.savedAt) {
            void flushRemote();
        }
    });
}
