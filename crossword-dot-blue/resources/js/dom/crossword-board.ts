import type { Puzzle } from "@bindings/Puzzle";
import { Game } from "../game/game";
import { attachGrid } from "./grid-view";
import { attachClueList } from "./clue-list-view";
import { attachClueBanner } from "./clue-banner";
import { attachActions } from "./actions";
import { attachKeyboard } from "./keyboard";
import { attachScreens } from "./screens";
import { attachPersistence } from "./persistence";
import { attachTimer } from "./timer";

export class CrosswordBoard extends HTMLElement {
    game?: Game;

    connectedCallback(): void {
        const raw = this.querySelector('script[type="application/json"]')?.textContent;
        const svg = this.querySelector<SVGSVGElement>("svg.grid");
        if (!raw || !svg) {
            console.warn("[crossword-board] missing puzzle json script or svg.grid child");
            return;
        }

        const puzzle: Puzzle = JSON.parse(raw);
        const game = new Game(puzzle);
        this.game = game;

        const root = this.closest<HTMLElement>("article.crossword") ?? this;
        const key = root.dataset.key ?? "";

        attachGrid(game, svg);
        attachClueList(game, root);
        attachClueBanner(game, root);
        attachKeyboard(game, root, this);
        const screens = attachScreens(game, root, this);
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
    }
}
