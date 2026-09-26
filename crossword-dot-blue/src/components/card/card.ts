import { storageKey, type StoredGame } from "../crossword/_game/stored";
import { formatDuration } from "../crossword/_game/time";

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

export class CwCard extends HTMLElement {
    connectedCallback(): void {
        this.refresh();
    }

    refresh(): void {
        const key = this.getAttribute("key");
        if (!key) return;

        const stored = read(key);
        const status = stored ? statusOf(stored) : null;
        const label = this.querySelector<HTMLElement>("[data-progress]");

        this.classList.remove("solved", "in-progress");
        if (!status) {
            label?.setAttribute("hidden", "");
            return;
        }

        this.classList.add(status.kind);
        if (label) {
            label.textContent = status.label;
            label.removeAttribute("hidden");
        }
    }
}

if (!customElements.get("cw-card")) {
    customElements.define("cw-card", CwCard);
}

window.addEventListener("pageshow", () => {
    for (const card of document.querySelectorAll<CwCard>("cw-card")) card.refresh();
});
