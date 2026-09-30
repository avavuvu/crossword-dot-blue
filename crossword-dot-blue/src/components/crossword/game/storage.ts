import type { Puzzle } from "@bindings/Puzzle";
import { isBlock } from "./puzzle";
import { completionOf } from "./reduce/entry";
import type { GameState } from "./state";
import { STORAGE_VERSION, type StoredGame } from "./stored";

const COMPLETIONS = new Set(["incomplete", "complete-but-wrong", "won"]);

export function serialize(puzzleKey: string, state: GameState, elapsedMs: number, previous?: StoredGame | null): StoredGame {
    return {
        version: STORAGE_VERSION,
        puzzleKey,
        savedAt: previous?.savedAt ?? 0,
        solvedAt: state.completion === "won" ? (previous?.solvedAt ?? null) : null,
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

export function isStoredGame(value: unknown): value is StoredGame {
    if (typeof value !== "object" || value === null) return false;
    const record = value as Record<string, unknown>;

    return (
        record.version === STORAGE_VERSION &&
        typeof record.puzzleKey === "string" &&
        typeof record.savedAt === "number" &&
        (record.solvedAt === null || typeof record.solvedAt === "number") &&
        typeof record.cursor === "number" &&
        typeof record.elapsedMs === "number" &&
        (record.direction === "across" || record.direction === "down") &&
        typeof record.completion === "string" &&
        COMPLETIONS.has(record.completion) &&
        Array.isArray(record.entries) &&
        record.entries.every((entry) => typeof entry === "string") &&
        Array.isArray(record.checked) &&
        record.checked.every(
            (pair) => Array.isArray(pair) && pair.length === 2 && typeof pair[0] === "number" && typeof pair[1] === "boolean",
        )
    );
}
