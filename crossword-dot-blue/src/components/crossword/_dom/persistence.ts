import type { Game } from "../_game/game";
import { deserialize, isEmpty, serialize, storageKey, type StoredGame } from "../_game/storage";
import type { Timer } from "./timer";

const SAVE_DELAY_MS = 300;

function read(key: string): StoredGame | null {
    try {
        const raw = localStorage.getItem(key);
        return raw ? (JSON.parse(raw) as StoredGame) : null;
    } catch (error) {
        console.warn("[persistence] read failed", error);
        return null;
    }
}

function write(key: string, value: StoredGame | null): void {
    try {
        if (value) localStorage.setItem(key, JSON.stringify(value));
        else localStorage.removeItem(key);
    } catch (error) {
        console.warn("[persistence] write failed", error);
    }
}

export function attachPersistence(game: Game, puzzleKey: string, timer: Timer): boolean {
    const key = storageKey(puzzleKey);
    let previous = read(key);
    let pending: ReturnType<typeof setTimeout> | undefined;

    const flush = () => {
        clearTimeout(pending);
        pending = undefined;

        if (isEmpty(game.state) && timer.elapsed() === 0) {
            previous = null;
            write(key, null);
            return;
        }

        previous = serialize(puzzleKey, game.state, timer.elapsed(), previous);
        write(key, previous);
    };

    const schedule = () => {
        clearTimeout(pending);
        pending = setTimeout(flush, SAVE_DELAY_MS);
    };

    game.on("entry", schedule);
    game.on("check", schedule);
    game.on("cursor", schedule);
    game.on("completion", flush);
    game.on("reset", flush);

    document.addEventListener("visibilitychange", () => {
        if (document.visibilityState === "hidden") flush();
    });
    window.addEventListener("pagehide", flush);

    const restored = previous ? deserialize(game.puzzle, puzzleKey, previous) : null;
    if (restored && previous) {
        timer.setElapsed(previous.elapsedMs);
        game.dispatch({ type: "restore", state: restored });
        return true;
    }

    if (previous) write(key, null);
    return false;
}
