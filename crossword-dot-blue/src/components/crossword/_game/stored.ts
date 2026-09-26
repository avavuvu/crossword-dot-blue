import type { Direction } from "@bindings/Direction";
import type { CompletionState } from "./events";

export const STORAGE_VERSION = 1;

export type StoredGame = {
    version: typeof STORAGE_VERSION;
    puzzleKey: string;
    savedAt: number;
    solvedAt: number | null;
    entries: string[];
    checked: [number, boolean][];
    cursor: number;
    direction: Direction;
    completion: CompletionState;
    elapsedMs: number;
};

export function storageKey(puzzleKey: string): string {
    return `crossword:v${STORAGE_VERSION}:${puzzleKey}`;
}
