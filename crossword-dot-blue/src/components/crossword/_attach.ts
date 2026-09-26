import type { Puzzle } from "@bindings/Puzzle";
import { Game } from "./_game/game";
import { attachGrid } from "./_dom/grid-view";
import { attachClueList } from "./_dom/clue-list-view";
import { attachClueBanner } from "./_dom/clue-banner";
import { attachActions } from "./_dom/actions";
import { attachKeyboard } from "./_dom/keyboard";
import { attachScreens } from "./_dom/screens";
import { attachPersistence } from "./_dom/persistence";
import { attachTimer } from "./_dom/timer";

export function attach(root: HTMLElement): Game | null {
    const raw = root.querySelector('script[type="application/json"]')?.textContent;
    const svg = root.querySelector<SVGSVGElement>("svg.grid");
    const board = root.querySelector<HTMLElement>(".frame");
    if (!raw || !svg || !board) {
        console.warn("[cw-crossword] missing puzzle json, svg.grid or .frame");
        return null;
    }

    const puzzle: Puzzle = JSON.parse(raw);
    const game = new Game(puzzle);
    const key = root.getAttribute("key") ?? "";

    attachGrid(game, svg);
    attachClueList(game, root);
    attachClueBanner(game, root);
    attachKeyboard(game, root, board);
    const screens = attachScreens(game, root, board);
    const timer = attachTimer(game, root);

    attachActions(game, root, {
        start() {
            screens.start();
            timer.start();
        },
        dismiss: () => screens.dismiss(),
    });

    const restored = key ? attachPersistence(game, key, timer) : false;
    if (restored) screens.markResumed();

    return game;
}
