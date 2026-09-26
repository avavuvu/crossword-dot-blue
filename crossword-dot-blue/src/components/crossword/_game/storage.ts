import type { Direction } from "@bindings/Direction";
import type { Puzzle } from "@bindings/Puzzle";
import type { CompletionState } from "./events";
import { isBlock } from "./puzzle";
import { completionOf } from "./reduce/entry";
import type { GameState } from "./state";
import { STORAGE_VERSION, type StoredGame, storageKey } from "./stored";

export { type StoredGame, storageKey };

export function serialize(puzzleKey: string, state: GameState, elapsedMs: number, previous?: StoredGame | null): StoredGame {
    const now = Date.now();
    const solvedAt = state.completion === "won" ? (previous?.solvedAt ?? now) : null;

    return {
        version: STORAGE_VERSION,
        puzzleKey,
        savedAt: now,
        solvedAt,
        entries: [...state.entries],
        checked: [...state.checked.entries()],
        cursor: state.cursor,
        direction: state.direction,
        completion: state.completion,
        elapsedMs: Math.max(0, Math.floor(elapsedMs)),
    };
}

export function deserialize(puzzle: Puzzle, puzzleKey: string, raw: unknown): GameState | null {
    if (!isStoredGame(raw)) return null;
    if (raw.puzzleKey !== puzzleKey) return null;
    if (raw.entries.length !== puzzle.cells.length) return null;
    if (raw.cursor < 0 || raw.cursor >= puzzle.cells.length || isBlock(puzzle, raw.cursor)) return null;
    if (raw.checked.some(([index]) => index < 0 || index >= puzzle.cells.length)) return null;

    return {
        cursor: raw.cursor,
        direction: raw.direction,
        entries: raw.entries,
        checked: new Map(raw.checked),
        rebus: false,
        completion: completionOf(puzzle, raw.entries),
    };
}

export function isEmpty(state: GameState): boolean {
    return state.entries.every((entry) => entry === "") && state.checked.size === 0;
}

function isStoredGame(value: unknown): value is StoredGame {
    if (typeof value !== "object" || value === null) return false;
    const v = value as Record<string, unknown>;

    return (
        v.version === STORAGE_VERSION &&
        typeof v.puzzleKey === "string" &&
        typeof v.cursor === "number" &&
        typeof v.elapsedMs === "number" &&
        (v.direction === "across" || v.direction === "down") &&
        Array.isArray(v.entries) &&
        v.entries.every((e) => typeof e === "string") &&
        Array.isArray(v.checked) &&
        v.checked.every(
            (pair) => Array.isArray(pair) && pair.length === 2 && typeof pair[0] === "number" && typeof pair[1] === "boolean",
        )
    );
}
