import type { Game } from "../_game/game";
import { clueIdAt, opposite } from "../_game/puzzle";

export function attachClueList(game: Game, root: HTMLElement): void {
    const buttons = new Map<string, HTMLButtonElement>();
    for (const button of root.querySelectorAll<HTMLButtonElement>("button[data-clue]")) {
        buttons.set(button.dataset.clue!, button);
    }

    let active: HTMLButtonElement | undefined;
    let crossing: HTMLButtonElement | undefined;
    let referenced: HTMLButtonElement[] = [];

    const paint = () => {
        active?.classList.remove("active");
        active?.removeAttribute("aria-current");
        crossing?.classList.remove("crossing");
        for (const button of referenced) button.classList.remove("referenced");

        const { cursor, direction } = game.state;
        const activeId = clueIdAt(game.puzzle, cursor, direction);
        const crossingId = clueIdAt(game.puzzle, cursor, opposite(direction));
        const refs = activeId ? game.puzzle.clues[activeId]?.refs ?? [] : [];

        active = activeId ? buttons.get(activeId) : undefined;
        crossing = crossingId ? buttons.get(crossingId) : undefined;
        referenced = refs.flatMap((id) => buttons.get(id) ?? []);

        active?.classList.add("active");
        active?.setAttribute("aria-current", "true");
        crossing?.classList.add("crossing");
        for (const button of referenced) button.classList.add("referenced");
        active?.scrollIntoView({ block: "nearest" });
    };

    game.on("cursor", paint);
    game.on("restore", paint);

    root.addEventListener("click", (event) => {
        const button = (event.target as Element).closest<HTMLButtonElement>("button[data-clue]");
        if (button && buttons.has(button.dataset.clue!)) {
            game.dispatch({ type: "select-clue", id: button.dataset.clue! });
        }
    });

    paint();
}
