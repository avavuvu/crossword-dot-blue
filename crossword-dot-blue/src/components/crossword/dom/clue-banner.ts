import type { Game } from "../game/game";
import { clueAt } from "../game/puzzle";

export type ClueBannerParts = {
    label: HTMLElement;
    text: HTMLElement;
    preview: HTMLElement;
    hint: HTMLElement | undefined;
    rebus: HTMLElement | undefined;
    menus: HTMLElement[];
    clues: Map<string, HTMLButtonElement>;
};

export function attachClueBanner(game: Game, parts: ClueBannerParts): void {
    const { label, text, preview, hint, rebus, menus, clues } = parts;

    const renderedBody = (id: string) => clues.get(id)?.querySelector<HTMLElement>(".body")?.innerHTML;

    const paintClue = () => {
        const { cursor, direction } = game.state;
        const clue = clueAt(game.puzzle, cursor, direction);
        label.textContent = clue ? `${clue.number}${clue.direction === "across" ? "A" : "D"}` : "";
        const html = clue ? renderedBody(clue.id) : undefined;
        if (html !== undefined) text.innerHTML = html;
        else text.textContent = clue?.body ?? "";
        if (hint) hint.hidden = !clue?.hint;
    };

    const paintPreview = () => {
        const { cursor, direction, entries } = game.state;
        const clue = clueAt(game.puzzle, cursor, direction);

        const letters = clue?.indexes.map((clueIndex, arrayIndex) => {
            const letter = entries[clueIndex] || "_";
            return clue.splits.includes(arrayIndex + 1) ? `${letter} ` : letter;
        }) ?? [];

        preview.textContent = letters.join("");
    };

    game.on("cursor", () => {
        paintClue();
        paintPreview();
    });
    game.on("entry", paintPreview);
    game.on("restore", () => {
        paintClue();
        paintPreview();
        rebus?.setAttribute("aria-pressed", String(game.state.rebus));
    });
    game.on("mode", ({ rebus: on }) => rebus?.setAttribute("aria-pressed", String(on)));

    if (!CSS.supports("position-area", "bottom")) {
        for (const menu of menus) {
            menu.addEventListener("beforetoggle", (event) => {
                if ((event as ToggleEvent).newState !== "open") return;
                const trigger = menu.previousElementSibling;
                if (!trigger) return;
                const rect = trigger.getBoundingClientRect();
                menu.style.setProperty("--menu-top", `${rect.bottom + 4}px`);
                menu.style.setProperty("--menu-left", `${rect.left}px`);
            });
        }
    }

    paintClue();
    paintPreview();
}
