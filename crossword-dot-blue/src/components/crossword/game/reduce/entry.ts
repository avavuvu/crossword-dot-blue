import type { Puzzle } from "@bindings/Puzzle";
import type { CompletionState } from "../events";
import type { GameState } from "../state";
import { clueAt, letterIndexes, matches } from "../puzzle";
import { moveTo } from "./cursor";

/** writes one cell. returns the same state when the cell is locked by a correct check */
export function setEntry(puzzle: Puzzle, state: GameState, index: number, value: string): GameState {
    if (state.checked.get(index) === true) return state;
    if (state.entries[index] === value) return state;

    const entries = state.entries.with(index, value);

    const checked = new Map(state.checked);
    checked.delete(index);

    return { ...state, entries, checked, completion: completionOf(puzzle, entries) };
}

export function completionOf(puzzle: Puzzle, entries: readonly string[]): CompletionState {
    const letters = letterIndexes(puzzle);
    if (letters.some((index) => entries[index] === "")) return "incomplete";
    return letters.every((index) => matches(puzzle, index, entries[index]!)) ? "won" : "complete-but-wrong";
}

export function input(puzzle: Puzzle, state: GameState, char: string): GameState {
    const clue = clueAt(puzzle, state.cursor, state.direction);
    if (!clue) return state;

    if (state.rebus) {
        return setEntry(puzzle, state, state.cursor, state.entries[state.cursor] + char);
    }

    const wasEmpty = state.entries[state.cursor] === "";
    const written = setEntry(puzzle, state, state.cursor, char);

    const position = clue.indexes.indexOf(state.cursor);
    const rest = clue.indexes.slice(position + 1);
    if (rest.length === 0) return written;

    const next = wasEmpty ? rest.find((index) => written.entries[index] === "") ?? rest[0]! : rest[0]!;

    return moveTo(written, next, state.direction);
}

export function backspace(puzzle: Puzzle, state: GameState): GameState {
    const clue = clueAt(puzzle, state.cursor, state.direction);
    if (!clue) return state;

    const current = state.entries[state.cursor]!;
    if (current !== "") {
        const value = state.rebus ? current.slice(0, -1) : "";
        const cleared = setEntry(puzzle, state, state.cursor, value);
        if (cleared !== state) return cleared;
    }

    const position = clue.indexes.indexOf(state.cursor);
    if (position <= 0) return state;

    const previous = clue.indexes[position - 1]!;
    const moved = moveTo(state, previous, state.direction);
    return setEntry(puzzle, moved, previous, "");
}
