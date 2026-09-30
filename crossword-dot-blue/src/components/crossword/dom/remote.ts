import { isStoredGame } from "../game/storage";
import type { StoredGame } from "../game/stored";

export type SaveResult =
    | { kind: "saved"; savedAt: number; solvedAt: number | null }
    | { kind: "stale"; theirs: StoredGame }
    | { kind: "failed" };

export type Remote = {
    load(): Promise<StoredGame | null>;
    save(value: StoredGame, keepalive?: boolean): Promise<SaveResult>;
    clear(keepalive?: boolean): Promise<void>;
};

export function remoteStore(puzzleKey: string): Remote {
    const share = new URLSearchParams(location.search).get("share");
    const url = `/progress/${puzzleKey}${share ? `?share=${encodeURIComponent(share)}` : ""}`;

    return {
        async load() {
            try {
                const response = await fetch(url, { credentials: "same-origin" });
                if (response.status !== 200) return null;
                const body: unknown = await response.json();
                return isStoredGame(body) ? body : null;
            } catch {
                return null;
            }
        },

        async save(value, keepalive = false) {
            try {
                const response = await fetch(url, {
                    method: "PUT",
                    credentials: "same-origin",
                    headers: { "content-type": "application/json" },
                    body: JSON.stringify(value),
                    keepalive,
                });

                if (response.status === 409) {
                    const theirs: unknown = await response.json();
                    return isStoredGame(theirs) ? { kind: "stale", theirs } : { kind: "failed" };
                }

                if (!response.ok) return { kind: "failed" };

                const body = (await response.json()) as { savedAt?: unknown; solvedAt?: unknown };
                if (typeof body.savedAt !== "number") return { kind: "failed" };
                return { kind: "saved", savedAt: body.savedAt, solvedAt: typeof body.solvedAt === "number" ? body.solvedAt : null };
            } catch {
                return { kind: "failed" };
            }
        },

        async clear(keepalive = false) {
            try {
                await fetch(url, { method: "DELETE", credentials: "same-origin", keepalive });
            } catch {}
        },
    };
}
