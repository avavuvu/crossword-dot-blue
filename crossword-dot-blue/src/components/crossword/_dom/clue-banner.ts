import type { Game } from "../_game/game";
import { clueAt } from "../_game/puzzle";

export function attachClueBanner(game: Game, host: HTMLElement): void {
    const label = host.querySelector<HTMLElement>("[data-clue-label]");
    const text = host.querySelector<HTMLElement>("[data-clue-text]");
    const rebus = host.querySelector<HTMLElement>('[data-action="toggle-rebus"]');
    const entryPreview = host.querySelector<HTMLElement>("[data-entry-preview]");
    const hintButton = host.querySelector<HTMLElement>("[data-hint-button]");

    const renderedBody = (id: string) =>
        host.querySelector<HTMLElement>(`button[data-clue="${id}"] .body`)?.innerHTML;

    const paintClue = () => {
        const { cursor, direction } = game.state;
        const clue = clueAt(game.puzzle, cursor, direction);
        if (label) label.textContent = clue ? `${clue.number}${clue.direction === "across" ? "A" : "D"}` : "";
        if (text) {
            const html = clue ? renderedBody(clue.id) : undefined;
            if (html !== undefined) text.innerHTML = html;
            else text.textContent = clue?.body ?? "";
        }
        if (hintButton) hintButton.hidden = !clue?.hint;
    };

    const paintPreview = () => {
        if (!entryPreview) return;

        const { cursor, direction, entries } = game.state;
        const clue = clueAt(game.puzzle, cursor, direction);

        const preview = clue?.indexes.map((clueIndex, arrayIndex) => {
            const letter = entries[clueIndex] || "_";
            return clue.splits.includes(arrayIndex + 1) ? `${letter} ` : letter;
        }) ?? [];

        entryPreview.textContent = preview.join("");
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
        for (const menu of host.querySelectorAll<HTMLElement>(".dropdown > [popover]")) {
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
