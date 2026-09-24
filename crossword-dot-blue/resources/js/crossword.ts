import { CrosswordBoard } from "./dom/crossword-board";

if (!customElements.get("crossword-board")) {
    customElements.define("crossword-board", CrosswordBoard);
}
