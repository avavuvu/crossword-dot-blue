import type { Direction } from "@bindings/Direction";
import type { Puzzle } from "@bindings/Puzzle";
import type { GameState } from "../state";
import { clueAt, clueIdAt, isBlock, nextClue, opposite } from "../puzzle";

export function moveTo(state: GameState, cursor: number, direction: Direction): GameState {
    if (state.cursor === cursor && state.direction === direction) return state;
    return { ...state, cursor, direction };
}

/** keeps the current direction when the cell has a word in it, otherwise turns */
export function directionFor(puzzle: Puzzle, index: number, preferred: Direction): Direction {
    if (clueIdAt(puzzle, index, preferred)) return preferred;
    if (clueIdAt(puzzle, index, opposite(preferred))) return opposite(preferred);
    return preferred;
}

export function select(puzzle: Puzzle, state: GameState, index: number): GameState {
    if (isBlock(puzzle, index)) return state;

    if (index === state.cursor) return toggleDirection(puzzle, state);

    return moveTo(state, index, directionFor(puzzle, index, state.direction));
}

export function toggleDirection(puzzle: Puzzle, state: GameState): GameState {
    const turned = opposite(state.direction);
    if (!clueIdAt(puzzle, state.cursor, turned)) return state;
    return moveTo(state, state.cursor, turned);
}

export function move(puzzle: Puzzle, state: GameState, dx: number, dy: number, arrowsTurn = false): GameState {
    const horizontal = dx !== 0;
    if (arrowsTurn && horizontal !== (state.direction === "across")) {
        return toggleDirection(puzzle, state);
    }

    const { width, height } = puzzle;
    let x = state.cursor % width;
    let y = Math.floor(state.cursor / width);

    for (let step = 0; step < width * height; step++) {
        x = (x + dx + width) % width;
        y = (y + dy + height) % height;
        const index = y * width + x;
        if (!isBlock(puzzle, index)) {
            return moveTo(state, index, directionFor(puzzle, index, state.direction));
        }
    }

    return state;
}

export function selectClue(puzzle: Puzzle, state: GameState, id: string, options = { toFirstEmpty: false }): GameState {
    const clue = puzzle.clues[id];
    if (!clue) return state;

    const target = options.toFirstEmpty
        ? clue.indexes.find((index) => state.entries[index] === "") ?? clue.indexes[0]!
        : clue.indexes[0]!;

    return moveTo(state, target, clue.direction);
}

export function cycleClue(puzzle: Puzzle, state: GameState, delta: 1 | -1): GameState {
    const current = clueAt(puzzle, state.cursor, state.direction);
    if (!current) return state;

    let clue = current;
    for (let step = 0; step < puzzle.clueOrder.length; step++) {
        clue = nextClue(puzzle, clue.id, delta);
        const empty = clue.indexes.find((index) => state.entries[index] === "");
        if (empty !== undefined) return moveTo(state, empty, clue.direction);
    }

    // every cell is filled, fall back to the next clue regardless
    const fallback = nextClue(puzzle, current.id, delta);
    return moveTo(state, fallback.indexes[0]!, fallback.direction);
}
