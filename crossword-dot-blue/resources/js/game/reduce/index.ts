import type { Puzzle } from "@bindings/Puzzle";
import type { Action } from "../actions";
import { initialState, type GameState } from "../state";
import { check, hint, reveal } from "./assist";
import { cycleClue, move, select, selectClue, toggleDirection } from "./cursor";
import { backspace, input } from "./entry";

export type ReduceOptions = {
    arrowsTurn: boolean;
    random: (count: number) => number;
};

export const defaultOptions: ReduceOptions = {
    arrowsTurn: false,
    random: (count) => Math.floor(Math.random() * count),
};

export function reduce(puzzle: Puzzle, state: GameState, action: Action, options = defaultOptions): GameState {
    switch (action.type) {
        case "input":
            return input(puzzle, state, action.char);
        case "backspace":
            return backspace(puzzle, state);
        case "move":
            return move(puzzle, state, action.dx, action.dy, options.arrowsTurn);
        case "select":
            return select(puzzle, state, action.index);
        case "select-clue":
            return selectClue(puzzle, state, action.id, { toFirstEmpty: true });
        case "next-clue":
            return cycleClue(puzzle, state, action.delta);
        case "toggle-direction":
            return toggleDirection(puzzle, state);
        case "toggle-rebus":
            return { ...state, rebus: !state.rebus };
        case "check":
            return check(puzzle, state, action.scope);
        case "reveal":
            return reveal(puzzle, state, action.scope);
        case "hint":
            return hint(puzzle, state, options.random);
        case "reset":
            return initialState(puzzle);
        case "restore":
            return action.state;
    }
}
