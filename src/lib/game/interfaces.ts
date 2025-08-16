import type { Coord } from "./Coord"
import type { Clue } from "./types"



export interface GridManagerInterface {
    inputChar(char: string): void
    backspace(): void
    moveBy([deltaX, deltaY]: Coord): void
    /**
     * @returns true is successfully edited, returns false if the cell was checked to be correct and therefore unchangable
     */
    setGridCell(cellIndex: number, char: string): boolean 
}

interface SetClueOptions {
    preserveCurrentCursorIndex: boolean,
    newIndex: number,
    indexToNextEmpty: boolean
}

export interface ClueManagerInterface {
    setClue(newClue: Clue, options: Partial<SetClueOptions>): void
    cycleClues(change: 1 | -1): void
    toggleOrientation(): void
}