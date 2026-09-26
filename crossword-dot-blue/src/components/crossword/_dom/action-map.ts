import type { UiAction } from "@bindings/UiAction";
import type { Action } from "../_game/actions";

function fromUiAction(name: UiAction): Action {
    switch (name) {
        case "start":
            return { type: "start" };
        case "dismiss":
            return { type: "dismiss" };
        case "reset":
            return { type: "reset" };
        case "hint":
            return { type: "hint" };
        case "prev-clue":
            return { type: "next-clue", delta: -1 };
        case "next-clue":
            return { type: "next-clue", delta: 1 };
        case "toggle-direction":
            return { type: "toggle-direction" };
        case "toggle-rebus":
            return { type: "toggle-rebus" };
        case "check-cell":
            return { type: "check", scope: "cell" };
        case "check-word":
            return { type: "check", scope: "word" };
        case "check-puzzle":
            return { type: "check", scope: "puzzle" };
        case "reveal-cell":
            return { type: "reveal", scope: "cell" };
        case "reveal-word":
            return { type: "reveal", scope: "word" };
        case "reveal-puzzle":
            return { type: "reveal", scope: "puzzle" };
        default: {
            const unhandled: never = name;
            throw new Error(`unhandled ui action ${String(unhandled)}`);
        }
    }
}

export function actionsFromAttribute(value: string): Array<Action> {
    return value
        .split(" ")
        .filter((name) => name.length > 0)
        .map((name) => fromUiAction(name as UiAction));
}

export function actionFromKeyboardEvent(event: KeyboardEvent): Action | null {
    if (event.metaKey || event.ctrlKey || event.altKey) return null;

    switch (event.key) {
        case "ArrowLeft":
            return { type: "move", dx: -1, dy: 0 };
        case "ArrowRight":
            return { type: "move", dx: 1, dy: 0 };
        case "ArrowUp":
            return { type: "move", dx: 0, dy: -1 };
        case "ArrowDown":
            return { type: "move", dx: 0, dy: 1 };
        case "Backspace":
            return { type: "backspace" };
        case "Tab":
            return { type: "next-clue", delta: event.shiftKey ? -1 : 1 };
        case " ":
            return { type: "toggle-direction" };
        case "Escape":
            return { type: "toggle-rebus" };
        case "Enter":
            return { type: "toggle-rebus" };
    }

    if (event.key.length === 1 && /[a-z0-9]/i.test(event.key)) {
        return { type: "input", char: event.key.toUpperCase() };
    }

    return null;
}

export function actionFromKey(key: string): Action | null {
    switch (key) {
        case "Backspace":
            return { type: "backspace" };
        case "Rebus":
            return { type: "toggle-rebus" };
        case "Direction":
            return { type: "toggle-direction" };
    }
    return key.length === 1 ? { type: "input", char: key.toUpperCase() } : null;
}
