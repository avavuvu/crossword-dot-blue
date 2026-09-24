import type { Direction } from "@bindings/Direction";
import type { Puzzle } from "@bindings/Puzzle";
import type { CompletionState } from "./events";

export type GameState = {
    readonly cursor: number;
    readonly direction: Direction;
    readonly entries: readonly string[];
    readonly checked: ReadonlyMap<number, boolean>;
    readonly rebus: boolean;
    readonly completion: CompletionState;
};

export function initialState(puzzle: Puzzle): GameState {
    const firstLetter = puzzle.cells.findIndex((cell) => cell.type === "letter");

    return {
        cursor: firstLetter === -1 ? 0 : firstLetter,
        direction: "across",
        entries: puzzle.cells.map(() => ""),
        checked: new Map(),
        rebus: false,
        completion: "incomplete",
    };
}
