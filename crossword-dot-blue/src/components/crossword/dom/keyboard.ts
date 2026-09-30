import type { Game } from "../game/game";
import { actionFromKey, actionFromKeyboardEvent } from "./action-map";

const INTERACTIVE = "button, a, input, select, textarea, [popover]";
const NATIVE_KEYS = new Set(["Enter", " ", "Tab", "Escape"]);

const REPEAT_DELAY_MS = 400;
const REPEAT_INTERVAL_MS = 60;

function attachOnscreen(game: Game, keyboard: HTMLElement, rebus: HTMLElement | undefined): void {
    let repeat: ReturnType<typeof setTimeout> | undefined;

    const stopRepeat = () => {
        clearTimeout(repeat);
        clearInterval(repeat);
        repeat = undefined;
    };

    keyboard.addEventListener("pointerdown", (event) => {
        const key = (event.target as Element).closest<HTMLElement>("[data-key]");
        if (!key) return;

        event.preventDefault();
        const action = actionFromKey(key.dataset.key!);
        if (!action) return;

        game.dispatch(action);

        if (action.type === "backspace") {
            repeat = setTimeout(() => {
                repeat = setInterval(() => game.dispatch(action), REPEAT_INTERVAL_MS);
            }, REPEAT_DELAY_MS);
        }
    });

    for (const name of ["pointerup", "pointercancel", "pointerleave"]) {
        keyboard.addEventListener(name, stopRepeat);
    }

    game.on("mode", ({ rebus: on }) => rebus?.setAttribute("aria-pressed", String(on)));
    game.on("restore", () => rebus?.setAttribute("aria-pressed", String(game.state.rebus)));
}

export function attachKeyboard(
    game: Game,
    root: HTMLElement,
    board: HTMLElement,
    intro: HTMLElement,
    onscreen: HTMLElement,
    rebusKey: HTMLElement | undefined,
): void {
    if (!board.hasAttribute("tabindex")) board.tabIndex = 0;

    attachOnscreen(game, onscreen, rebusKey);

    root.addEventListener("keydown", (event) => {
        if (!intro.hidden) return;

        const target = event.target as Element;
        const onControl = target !== board && target.closest(INTERACTIVE);
        if (onControl && NATIVE_KEYS.has(event.key)) return;

        const action = actionFromKeyboardEvent(event);
        if (!action) return;
        event.preventDefault();
        game.dispatch(action);
    });

    root.addEventListener("click", (event) => {
        if ((event.target as Element).closest(INTERACTIVE)) return;
        board.focus();
    });
}
