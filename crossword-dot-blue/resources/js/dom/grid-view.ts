import type { Game } from "../game/game";
import { clueAt } from "../game/puzzle";

export function attachGrid(game: Game, svg: SVGSVGElement): void {
    const cells = new Map<number, SVGElement>();
    for (const el of svg.querySelectorAll<SVGElement>("[data-index]")) {
        cells.set(Number(el.dataset.index), el);
    }

    const entries = new Map<number, SVGElement>();
    for (const el of svg.querySelectorAll<SVGElement>("[data-entry]")) {
        entries.set(Number(el.dataset.entry), el);
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
        const el = entries.get(index);
        if (!el) return;

        const value = game.state.entries[index] ?? "";
        const checked = game.state.checked.get(index);

        el.textContent = value;
        el.classList.toggle("rebus", value.length > 1);
        el.classList.toggle("correct", checked === true);
        el.classList.toggle("wrong", checked === false);
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
