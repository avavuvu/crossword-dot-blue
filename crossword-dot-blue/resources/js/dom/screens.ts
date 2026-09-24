import type { CompletionState } from "../game/events";
import type { Game } from "../game/game";

export type Screens = {
    start(): void;
    dismiss(): void;
    markResumed(): void;
};

export function attachScreens(game: Game, root: HTMLElement, board: HTMLElement): Screens {
    const intro = root.querySelector<HTMLElement>(".intro");
    const startButton = intro?.querySelector<HTMLElement>('[data-action="start"]');
    const screens = Array.from(root.querySelectorAll<HTMLElement>("[data-completion]"));

    const start = () => {
        if (!intro) return;
        intro.hidden = true;
        board.focus();
    };

    const hideAll = () => {
        for (const screen of screens) screen.hidden = true;
    };

    const dismiss = () => {
        hideAll();
        board.focus();
    };

    const show = (state: CompletionState) => {
        hideAll();
        const screen = screens.find((s) => s.dataset.completion === state);
        if (!screen) return;
        screen.hidden = false;
        screen.querySelector<HTMLElement>("h2")?.focus();
    };

    game.on("completion", ({ state }) => show(state));

    game.on("restore", () => {
        if (game.state.completion === "won") show("won");
    });

    return {
        start,
        dismiss,
        markResumed() {
            if (startButton) startButton.textContent = "Continue";
        },
    };
}
