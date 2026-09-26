import type { Puzzle } from "@bindings/Puzzle";
import type { Action } from "./actions";
import type { GameEventName, GameEvents } from "./events";
import { defaultOptions, reduce, type ReduceOptions } from "./reduce";
import { initialState, type GameState } from "./state";

export class Game extends EventTarget {
    readonly puzzle: Puzzle;
    options: ReduceOptions;
    #state: GameState;

    constructor(puzzle: Puzzle, options: Partial<ReduceOptions> = {}) {
        super();
        this.puzzle = puzzle;
        this.options = { ...defaultOptions, ...options };
        this.#state = initialState(puzzle);
    }

    get state(): GameState {
        return this.#state;
    }

    dispatch(action: Action): void {
        const before = this.#state;
        const after = reduce(this.puzzle, before, action, this.options);
        if (after === before) return;

        this.#state = after;

        if (action.type === "restore") {
            this.emit("restore", {});
            return;
        }

        this.#emitChanges(before, after);

        if (action.type === "reset") {
            this.emit("reset", {});
        }
    }

    on<K extends GameEventName>(
        name: K,
        listener: (detail: GameEvents[K]) => void,
        options?: AddEventListenerOptions,
    ): void {
        this.addEventListener(name, (event) => listener((event as CustomEvent<GameEvents[K]>).detail), options);
    }

    protected emit<K extends GameEventName>(name: K, detail: GameEvents[K]): void {
        this.dispatchEvent(new CustomEvent(name, { detail }));
    }

    #emitChanges(before: GameState, after: GameState): void {
        for (let index = 0; index < after.entries.length; index++) {
            if (before.entries[index] !== after.entries[index]) {
                this.emit("entry", { index, value: after.entries[index]! });
            }
        }

        if (before.checked !== after.checked) {
            for (const index of new Set([...before.checked.keys(), ...after.checked.keys()])) {
                const was = before.checked.get(index) ?? null;
                const now = after.checked.get(index) ?? null;
                if (was !== now) this.emit("check", { index, correct: now });
            }
        }

        if (before.rebus !== after.rebus) {
            this.emit("mode", { rebus: after.rebus });
        }

        if (before.cursor !== after.cursor || before.direction !== after.direction) {
            this.emit("cursor", { from: before.cursor, to: after.cursor, direction: after.direction });
        }

        if (before.completion !== after.completion) {
            this.emit("completion", { state: after.completion });
        }
    }
}
