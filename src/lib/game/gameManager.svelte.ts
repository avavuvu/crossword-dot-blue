import type { Cell, Crossword } from "./types"
import { getOrientation } from "./Coord"
import { AssistManager, type AssistManagerInterface } from "./assistManager"
import type { ClueManagerInterface, GridManagerInterface } from "./interfaces"
import { GameStateStore } from "./gameState.svelte"
import { GridManager } from "./gridManager"
import { ClueManager } from "./clueManager"
import { TimeManager, type TimeManagerInterface } from "./timeManager"
import { StorageManager, type StorageManagerInterface } from "./storageManager"

class GameManager {
    #grid!: Cell[]
    width!: number
    height!: number

    state!: GameStateStore 
    gridManager!: GridManagerInterface
    clueManager!: ClueManagerInterface
    assistManager!: AssistManagerInterface
    timeManager!: TimeManagerInterface
    storageManager!: StorageManagerInterface

    init(crossword: Crossword) {
        this.storageManager = new StorageManager(crossword.metadata.collection, crossword.metadata.id)
        const savedState = this.storageManager.get()

        if(savedState) {   
            this.state = GameStateStore.fromSavedState(savedState)
        } else {
            this.state = new GameStateStore(crossword)
        }
        
        this.#grid = crossword.grid
        this.width = crossword.width
        this.height = crossword.height

        this.clueManager = new ClueManager(this.state, crossword)
        this.gridManager = new GridManager(this.state, this.clueManager, this.storageManager, crossword)
        this.timeManager = new TimeManager(this.state)
        this.assistManager = new AssistManager(this.state, this.gridManager, this.timeManager, this.storageManager, crossword)
    }

    selectCell(index: number) {
        if(!this.state.clue) {
            const clue = this.#grid[index].clues?.['across'] || this.#grid[index].clues?.['down']
            this.state.clue = clue
            
            if(clue) {
                this.state.indexInClue = clue.indexes.indexOf(index)
            }

            return
        }

        const cursorIndex = this.state.clue.indexes[this.state.indexInClue]
        // change the orientation
        if(cursorIndex === index) {
            this.clueManager.toggleOrientation()
            return
        }

        // change the clue
        const clue = this.#grid[index].clues?.[getOrientation(this.state.clue.isHorizontal)] ||
            this.#grid[index].clues?.[getOrientation(!this.state.clue.isHorizontal)]
            
        // do this check avoids an uncessary update.
        if(clue) {
            this.clueManager.setClue(clue, { newIndex: index})
        }
    }
}

export const gameManager = new GameManager()