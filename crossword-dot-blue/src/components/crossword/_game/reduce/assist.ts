import type { Puzzle } from "@bindings/Puzzle";
import type { AssistScope } from "../actions";
import type { GameState } from "../state";
import { clueAt, letterIndexes, matches, solutionAt } from "../puzzle";
import { setEntry } from "./entry";

function scopeIndexes(puzzle: Puzzle, state: GameState, scope: AssistScope): number[] {
    switch (scope) {
        case "cell":
            return [state.cursor];
        case "word":
            return clueAt(puzzle, state.cursor, state.direction)?.indexes ?? [];
        case "puzzle":
            return letterIndexes(puzzle);
    }
}

export function check(puzzle: Puzzle, state: GameState, scope: AssistScope): GameState {
    const checked = new Map(state.checked);

    for (const index of scopeIndexes(puzzle, state, scope)) {
        const entry = state.entries[index]!;
        if (entry === "") continue;
        checked.set(index, matches(puzzle, index, entry));
    }

    return { ...state, checked };
}

export function revealCell(puzzle: Puzzle, state: GameState, index: number): GameState {
    const solution = solutionAt(puzzle, index);
    if (solution === null) return state;

    const unlocked = { ...state, checked: new Map(state.checked) };
    unlocked.checked.delete(index);

    const written = setEntry(puzzle, unlocked, index, solution);
    const checked = new Map(written.checked);
    checked.set(index, true);

    return { ...written, checked };
}

export function reveal(puzzle: Puzzle, state: GameState, scope: AssistScope): GameState {
    return scopeIndexes(puzzle, state, scope).reduce((next, index) => revealCell(puzzle, next, index), state);
}

export function hint(puzzle: Puzzle, state: GameState, pick: (count: number) => number): GameState {
    const clue = clueAt(puzzle, state.cursor, state.direction);
    if (!clue) return state;

    const empty = clue.indexes.filter((index) => state.entries[index] === "");
    if (empty.length === 0) return state;

    return revealCell(puzzle, state, empty[pick(empty.length)]!);
}
