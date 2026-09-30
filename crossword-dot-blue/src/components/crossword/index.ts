import type { Puzzle } from "@bindings/Puzzle";
import type { Crossword } from "@setups";
import { Game } from "./game/game";
import { attachGrid } from "./dom/grid-view";
import { attachClueList } from "./dom/clue-list-view";
import { attachClueBanner } from "./dom/clue-banner";
import { attachActions } from "./dom/actions";
import { attachKeyboard } from "./dom/keyboard";
import { attachScreens } from "./dom/screens";
import { attachPersistence } from "./dom/persistence";
import { attachTimer } from "./dom/timer";

export const crossword: Crossword = (root, refs) => {
    const svg = refs.board.querySelector<SVGSVGElement>("svg.grid");
    const raw = refs.puzzleJson.textContent;
    if (!raw || !svg) {
        console.warn("[crossword] missing puzzle json or svg.grid");
        return;
    }

    const puzzle: Puzzle = JSON.parse(raw);
    const game = new Game(puzzle);
    const key = root.getAttribute("key") ?? "";
    const clues = new Map(refs.clue.map((button) => [button.dataset.clue!, button]));

    attachGrid(game, svg);
    attachClueList(game, root, clues);
    attachClueBanner(game, {
        label: refs.clueLabel,
        text: refs.clueText,
        preview: refs.entryPreview,
        hint: refs.hintButton,
        rebus: refs.rebus,
        menus: refs.menu,
        clues,
    });
    attachKeyboard(game, root, refs.board, refs.intro, refs.keyboard, refs.rebusKey.at(0));
    const screens = attachScreens(game, refs.board, refs.intro, refs.options, refs.completion);
    const timer = attachTimer(game, refs.timer, refs.completionTime);

    attachActions(game, root, {
        start() {
            screens.start();
            timer.start();
        },
        dismiss: () => screens.dismiss(),
        options: () => screens.options(),
    });

    if (key) attachPersistence(game, key, timer);
};
