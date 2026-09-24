import type { Direction } from "@bindings/Direction";

export type CompletionState = "incomplete" | "complete-but-wrong" | "won";

export type GameEvents = {
    cursor: { from: number; to: number; direction: Direction };
    entry: { index: number; value: string };
    check: { index: number; correct: boolean | null };
    mode: { rebus: boolean };
    completion: { state: CompletionState };
    restore: {};
    reset: {};
};

export type GameEventName = keyof GameEvents;

export type GameEvent<K extends GameEventName> = CustomEvent<GameEvents[K]>;
