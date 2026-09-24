import type { Action, ActionType, AssistScope } from "../game/actions";

const getAction = (type: string, scope?: AssistScope): Action | null => {
    switch (type) {
        case "prev-clue":
            return { type: "next-clue", delta: -1 };
        case "next-clue":
            return { type: "next-clue", delta: 1 };
        case "toggle-rebus":
            return { type: "toggle-rebus" };
        case "toggle-direction":
            return { type: "toggle-direction" };
        case "check":
            return scope ? { type: "check", scope } : null;
        case "reveal":
            return scope ? { type: "reveal", scope } : null;
        default:
            return { type } as Action
    }
}

export function actionFromNames(name: string): Array<Action> {
    let actions: Array<Action> = []

    for (const actionName of name.split(" ")) {
        const [type, scope] = actionName.split(":") as [string, AssistScope | undefined];

        const action = getAction(type, scope)
        if (action) {
            actions.push(action)
        }
    }

    return actions
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
