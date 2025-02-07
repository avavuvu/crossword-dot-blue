import type { GameStateStore } from "./gameState.svelte";
import type { ClueManagerInterface, GridManagerInterface } from "./interfaces";
import type { StorageManagerInterface } from "./storageManager";
import type { TimeManager, TimeManagerInterface } from "./timeManager";
import type { Cell, Clue, Crossword } from "./types"

export interface AssistManagerInterface {
    checkSelected(): void
    checkWord(): void
    checkPuzzle(): void

    revealSelected(): void
    revealWord(): void
    revealPuzzle(): void

    resetPuzzle(): void

    getHint(): void
}

export class AssistManager implements AssistManagerInterface {
    state: GameStateStore
    gridManager: GridManagerInterface
    timeManager: TimeManagerInterface
    storageManager: StorageManagerInterface
    completedGrid: string[]
    board: Cell[]

    constructor(state: GameStateStore, gridManager: GridManagerInterface, timeManager: TimeManagerInterface, storageManager: StorageManagerInterface, crossword: Crossword) {
        this.state = state
        this.completedGrid = crossword.completedGrid
        this.board = crossword.grid
        this.gridManager = gridManager
        this.timeManager = timeManager
        this.storageManager = storageManager
    }

    checkCell(cellIndex: number): void {
        if(!this.state.grid[cellIndex]) {
            return
        }

        const isCorrect = this.completedGrid[cellIndex] === this.state.grid[cellIndex]
        this.state.checked.set(cellIndex, isCorrect)

        this.storageManager.set(this.state)
    }

    checkSelected(): void {
        if(!this.state.cursorIndex) { 
            return 
        }

        this.checkCell(this.state.cursorIndex)
    }

    checkWord(): void {
        if(!this.state.clue) { 
            return 
        }

        this.state.clue.indexes.forEach((cellIndex) => {
            this.checkCell(cellIndex)
        })
    }

    checkPuzzle(): void {
        this.state.grid.forEach((_, cellIndex) => {
            if(this.board[cellIndex].solid) {
                return
            }

            this.checkCell(cellIndex)
        })
    }

    revealCell(cellIndex: number): void {
        this.gridManager.setGridCell(cellIndex, this.completedGrid[cellIndex])
        this.state.checked.set(cellIndex, true)
        this.storageManager.set(this.state)
    }

    getHint(): void {
        if(!this.state.clue) {
            return
        }

        const emptyCells = this.state.clue.indexes.filter(cellIndex => this.state.grid[cellIndex] === "")
        if(emptyCells.length === 0) {
            return
        }

        const randomIndex = emptyCells[Math.floor(Math.random() * emptyCells.length)]

        this.revealCell(randomIndex)
    }

    revealSelected(): void {
        if(!this.state.cursorIndex) {
            return
        }

        this.revealCell(this.state.cursorIndex)
    }

    revealWord(): void {
        if(!this.state.clue) {
            return
        }

        this.state.clue.indexes.forEach((cellIndexes) => {
            this.revealCell(cellIndexes)
        })
    }
    
    revealPuzzle(): void {
        this.state.grid.forEach((_, cellIndex) => {
            if(this.board[cellIndex].solid) {
                return
            }

            this.revealCell(cellIndex)
        })
    }

    resetPuzzle(): void {
        this.state.grid = this.state.grid.map((_) => {
            this.state.checked.clear()
            return ""
        })

        this.state.completion = "incomplete"
        this.timeManager.reset(true)
        this.storageManager.set(this.state)
    }
}