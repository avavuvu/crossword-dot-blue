import "../../src/views/crossword/crossword.css";

import { CrosswordBoard } from "./dom/crossword-board";

if (!customElements.get("crossword-board")) {
    customElements.define("crossword-board", CrosswordBoard);
}
