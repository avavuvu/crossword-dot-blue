import type { CompletionState } from "../game/events";
import type { Game } from "../game/game";

export type Screens = {
    start(): void;
    dismiss(): void;
    options(): void;
};

export function attachScreens(
    game: Game,
    board: HTMLElement,
    intro: HTMLElement,
    options: HTMLElement | undefined,
    screens: HTMLElement[],
): Screens {
    const startButton = intro.querySelector<HTMLElement>('[data-action="start"]');

    const start = () => {
        intro.hidden = true;
        board.focus();
    };

    const hideAll = () => {
        for (const screen of screens) screen.hidden = true;
        if (options) options.hidden = true;
    };

    const dismiss = () => {
        hideAll();
        board.focus();
    };

    const show = (state: CompletionState) => {
        hideAll();
        const screen = screens.find((screen) => screen.dataset.completion === state);
        if (!screen) return;
        screen.hidden = false;
        screen.querySelector<HTMLElement>("h2")?.focus();
    };

    const toggleOptions = () => {
        if (!options) return;
        if (!options.hidden) {
            dismiss();
            return;
        }
        for (const screen of screens) screen.hidden = true;
        options.hidden = false;
        options.querySelector<HTMLElement>("h2")?.focus();
    };

    game.on("completion", ({ state }) => show(state));

    game.on("restore", () => {
        if (startButton) startButton.textContent = "Continue";
        if (game.state.completion === "won") show("won");
    });

    return { start, dismiss, options: toggleOptions };
}
