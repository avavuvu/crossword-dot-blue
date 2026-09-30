import type { Game } from "../game/game";
import { clueAt } from "../game/puzzle";

export function attachGrid(game: Game, svg: SVGSVGElement): void {
    const cells = new Map<number, SVGElement>();
    for (const element of svg.querySelectorAll<SVGElement>("[data-index]")) {
        cells.set(Number(element.dataset.index), element);
    }

    const entries = new Map<number, SVGElement>();
    for (const element of svg.querySelectorAll<SVGElement>("[data-entry]")) {
        entries.set(Number(element.dataset.entry), element);
    }

    let word: readonly number[] = [];

    const paintCursor = () => {
        for (const index of word) cells.get(index)?.classList.remove("word", "active");

        const { cursor, direction } = game.state;
        word = clueAt(game.puzzle, cursor, direction)?.indexes ?? [cursor];

        for (const index of word) cells.get(index)?.classList.add("word");
        cells.get(cursor)?.classList.add("active");
    };

    const paintEntry = (index: number) => {
        const element = entries.get(index);
        if (!element) return;

        const value = game.state.entries[index] ?? "";
        const checked = game.state.checked.get(index);

        element.textContent = value;
        element.classList.toggle("rebus", value.length > 1);
        element.classList.toggle("correct", checked === true);
        element.classList.toggle("wrong", checked === false);
    };

    game.on("cursor", paintCursor);
    game.on("entry", ({ index }) => paintEntry(index));
    game.on("check", ({ index }) => paintEntry(index));
    game.on("restore", () => {
        paintCursor();
        for (const index of entries.keys()) paintEntry(index);
    });

    svg.addEventListener("click", (event) => {
        const cell = (event.target as Element).closest<SVGElement>("[data-index]");
        if (cell) game.dispatch({ type: "select", index: Number(cell.dataset.index) });
    });

    paintCursor();
}
