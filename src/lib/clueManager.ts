import { getOrientation } from "./Coord";
import { orderClues } from "./formatCrossword";
import type { GameStateStore } from "./gameState.svelte";
import type { ClueManagerInterface } from "./interfaces";
import type { Clue, Crossword } from "./types";

export class ClueManager implements ClueManagerInterface {
    state: GameStateStore
    crossword: Crossword
    #orderedClues: Clue[]

    constructor(state: GameStateStore, crossword: Crossword) {
        this.state = state
        this.crossword = crossword
        this.#orderedClues = orderClues(crossword.clues, "ordinal", crossword.width)
    }

    setClue(newClue: Clue, options: Partial<{
        preserveCurrentCursorIndex: boolean,
        newIndex: number,
        indexToNextEmpty: boolean
    }> = {}) {
        const cursorIndex = this.state.cursorIndex

        //they are the same ignore
        if(this.state.clue?.word !== newClue.word) {
            this.state.clue = newClue
        }

        if(options.preserveCurrentCursorIndex) {
            this.state.indexInClue = newClue.indexes.indexOf(cursorIndex!)

            if(this.state.indexInClue !== -1) {
                return
            }
            
            console.warn("Index Not Found in Clue. Did something go wrong?")
        }

        if(options.newIndex) {
            this.state.indexInClue = newClue.indexes.indexOf(options.newIndex)
            return
        }

        if(options.indexToNextEmpty) {
            const emptyCell = newClue.indexes.findIndex(index => this.state.grid[index] === "")

            if(emptyCell !== -1) {
                this.state.indexInClue = emptyCell
                return
            }
        }

        this.state.indexInClue = 0
    }

    setClueIfEmpty(newClue: Clue): boolean {
        const emptyCell = newClue.indexes.find(index => this.state.grid[index] === "")

        if(!emptyCell) {
            return false
        }

        this.setClue(newClue, { newIndex: emptyCell })  
        return true
    }

    toggleOrientation() {
        if(!this.state.clue) { return }

        const oppositeClue = this.crossword.grid[this.state.cursorIndex!].clues?.[getOrientation(!this.state.clue.isHorizontal)]

        if(!oppositeClue) {
            return
        }

        this.setClue(oppositeClue, { preserveCurrentCursorIndex: true })
    }

    cycleClues(change: 1 | -1) {
        if(!this.state.clue) { 
            this.setClue(this.crossword.clues[0])
            return
        }

        const currentClueIndex = this.#orderedClues
            .findIndex((clue) => JSON.stringify(clue) === JSON.stringify(this.state.clue))
    
        let iterations = 0
        let newClueIndex = currentClueIndex
        do {
            newClueIndex = (newClueIndex + change + this.crossword.clues.length) % this.crossword.clues.length
            let newClue = this.#orderedClues[newClueIndex]

            if(this.setClueIfEmpty(newClue)) {
                break
            }

            iterations ++
        } while ( iterations < this.crossword.clues.length )

        // failsafe in case entire grid is filled
        if(iterations >= this.crossword.clues.length) {
            const clueIndex = (currentClueIndex + change + this.crossword.clues.length) % this.crossword.clues.length
            this.setClue(this.#orderedClues[clueIndex])
        }
    }
}