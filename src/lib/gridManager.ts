import { coordsToIndex, getOrientation, indexToCoords, type Coord } from "./Coord"
import type { GameStateStore } from "./gameState.svelte"
import type { ClueManagerInterface, GridManagerInterface } from "./interfaces"
import { options } from "./options.svelte"
import type { StorageManagerInterface } from "./storageManager"
import type { Crossword } from "./types"

export class GridManager implements GridManagerInterface {
    state: GameStateStore
    crossword: Crossword
    clueManager: ClueManagerInterface
    completedGrid: string[]
    storageManager: StorageManagerInterface

    constructor(state: GameStateStore, clueManager: ClueManagerInterface, storageManager: StorageManagerInterface, crossword: Crossword) {
        this.state = state
        this.crossword = crossword
        this.clueManager = clueManager
        this.completedGrid = crossword.completedGrid
        this.storageManager = storageManager
    }
    
    // im unsure if this should even be here, because it is not really the grid responsibility to be updating this stuff?
    // maybe there should be a seperate class called completion manager?
    updateCompletitionState() {
        const anyIncompleteSquares = this.state.grid.some((char, index) => {
            if(this.crossword.grid[index].solid) {
                return false
            }

            if(char === "") {
                return true
            }

            return false
        })

        if(anyIncompleteSquares) {
            return false
        }
        
        if(JSON.stringify(this.completedGrid) === JSON.stringify(this.state.grid)) {
            this.state.completion = "won"
        } else {
            this.state.completion = "completeButWrong"
        }

        this.storageManager.set(this.state)
    }

    inputChar(char: string) {
        if(!this.state.clue) { return }

        if(this.state.isRebusMode) {
            
            this.setGridCell(
                this.state.cursorIndex!, 
                this.state.grid[this.state.cursorIndex!] + char
            )

            return
        }

        const cursorIndex = this.state.clue.indexes[this.state.indexInClue]
        const originalSquareEmpty = this.state.grid[cursorIndex] === ""
        this.setGridCell(cursorIndex, char)

        // check if the word is over => meaning next cell is solid
        const newIndexInClue = this.state.indexInClue + 1

        if(newIndexInClue >= this.state.clue.indexes.length) {
            return
        }

        if(originalSquareEmpty) {
            const firstEmptyIndexInClue = this.state.clue.indexes.findIndex((positionIndex, index) => 
                this.state.indexInClue < index &&
                this.state.grid[positionIndex] === "")

            if(firstEmptyIndexInClue !== -1) {
                this.state.indexInClue = firstEmptyIndexInClue
            } else {
                this.state.indexInClue ++
            }
            
            return
        }

        this.state.indexInClue ++
    }

    setGridCell(cellIndex: number, char: string): boolean {
        const cellCorrect = this.state.checked.get(cellIndex)
        if(cellCorrect) {
            return false
        }

        this.state.grid[cellIndex] = char

        //removed it from the check list as well
        this.state.checked.delete(cellIndex)

        this.updateCompletitionState()

        this.storageManager.set(this.state)

        return true
    }

    backspace() {
        if(!this.state.clue) { return }

        // its actually important we take cursorIndex here, because we are going to change the cursor position
        const cursorIndex = this.state.clue.indexes[this.state.indexInClue]
        if(this.state.grid[cursorIndex] !== "") {
            let valueToSet = ""
            
            if(this.state.isRebusMode) {
                valueToSet = this.state.grid[cursorIndex].slice(0,-1)
            }

            const setSuccesfully = this.setGridCell(cursorIndex, valueToSet)

            if(setSuccesfully) {
                return
            }
        }

        const previousIndexInClue = this.state.indexInClue - 1
        if(previousIndexInClue < 0) {
            return
        }

        this.state.indexInClue--

        const newPositionIndex = this.state.clue.indexes[this.state.indexInClue]
        if(this.state.grid[newPositionIndex] !== "") {
            this.setGridCell(newPositionIndex, "")
        }
    }

    moveBy([deltaX, deltaY]: Coord) {
        if(!this.state.clue) { return }

        if(options.ArrowKeyChangeOrientation.setting) {
            if((deltaX === 1 || deltaX === -1) !== this.state.clue.isHorizontal) {
                this.clueManager.toggleOrientation()
                return
            }

        }
        
        const [x, y] = indexToCoords(this.state.cursorIndex!, this.crossword.width)

        let newCoords: Coord = [
            (x + deltaX + this.crossword.width) % this.crossword.width,
            (y + deltaY + this.crossword.height) % this.crossword.height
        ]

        while(this.crossword.grid[coordsToIndex(newCoords, this.crossword.width)].solid) {
            newCoords = [
                (newCoords[0] + deltaX + this.crossword.width) % this.crossword.width,
                (newCoords[1] + deltaY + this.crossword.height) % this.crossword.height
            ]
        }

        const newIndex = coordsToIndex(newCoords, this.crossword.width)

        const clue = this.crossword.grid[newIndex].clues[getOrientation(this.state.clue.isHorizontal)]
            || this.crossword.grid[newIndex].clues[getOrientation(!this.state.clue.isHorizontal)]

        if(clue) {
            this.clueManager.setClue(clue!, { newIndex, preserveCurrentCursorIndex: false } )
        }
    }
}