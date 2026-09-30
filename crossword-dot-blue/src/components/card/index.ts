import type { CardProgress } from "@setups";
import { storageKey, type StoredGame } from "../crossword/game/stored";
import { formatDuration } from "../crossword/game/time";

type Status = { kind: "solved"; label: string } | { kind: "in-progress"; label: string };

function read(puzzleKey: string): StoredGame | null {
    try {
        const raw = localStorage.getItem(storageKey(puzzleKey));
        return raw ? (JSON.parse(raw) as StoredGame) : null;
    } catch {
        return null;
    }
}

function statusOf(stored: StoredGame): Status | null {
    if (stored.solvedAt !== null || stored.completion === "won") {
        return { kind: "solved", label: `Solved in ${formatDuration(stored.elapsedMs)}` };
    }

    const filled = stored.entries.filter((entry) => entry !== "").length;
    if (filled === 0 && stored.elapsedMs === 0) return null;

    return { kind: "in-progress", label: "In progress" };
}

export const cardProgress: CardProgress = (card, { progress, signal }) => {
    const refresh = (): void => {
        const key = card.getAttribute("key");
        if (!key) return;

        const stored = read(key);
        const status = stored ? statusOf(stored) : null;
        card.classList.remove("solved", "in-progress");
        if (!status) {
            progress.hidden = true;
            return;
        }

        card.classList.add(status.kind);
        progress.textContent = status.label;
        progress.hidden = false;
    };

    refresh();
    window.addEventListener("pageshow", refresh, { signal });
};
