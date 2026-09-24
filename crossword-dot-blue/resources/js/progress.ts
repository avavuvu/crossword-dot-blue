import { storageKey, type StoredGame } from "./game/storage";
import { formatDuration } from "./game/time";

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

export function markProgress(root: ParentNode): void {
    for (const card of root.querySelectorAll<HTMLElement>("[data-puzzle-key]")) {
        const stored = read(card.dataset.puzzleKey!);
        const status = stored ? statusOf(stored) : null;
        const label = card.querySelector<HTMLElement>("[data-progress]");

        card.classList.remove("solved", "in-progress");
        if (!status) {
            label?.setAttribute("hidden", "");
            continue;
        }

        card.classList.add(status.kind);
        if (label) {
            label.textContent = status.label;
            label.removeAttribute("hidden");
        }
    }
}
