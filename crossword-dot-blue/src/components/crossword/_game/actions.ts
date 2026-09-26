import type { GameState } from "./state";

export type AssistScope = "cell" | "word" | "puzzle";

export type Action =
    | { type: "input"; char: string }
    | { type: "backspace" }
    | { type: "move"; dx: -1 | 0 | 1; dy: -1 | 0 | 1 }
    | { type: "select"; index: number }
    | { type: "select-clue"; id: string }
    | { type: "next-clue"; delta: 1 | -1 }
    | { type: "toggle-direction" }
    | { type: "toggle-rebus" }
    | { type: "check"; scope: AssistScope }
    | { type: "reveal"; scope: AssistScope }
    | { type: "hint" }
    | { type: "reset" }
    | { type: "restore"; state: GameState }
    | { type: "start"; }
    | { type: "dismiss"; };

export type ActionType = Action["type"];
