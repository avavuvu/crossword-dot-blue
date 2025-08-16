import { SvelteMap } from "svelte/reactivity";
import type { Clue, CompletionState, Crossword } from "./types"

interface GameState {
    clue: Clue | undefined,
    indexInClue: number,
    grid: string[]
    checked: SvelteMap<number,boolean>;
    completion: CompletionState
    elapsedTime: number
    isRebusMode: boolean
    
    get cursorIndex(): number | undefined
}

export type GameStateToSave = Omit<GameState,"clue" | "indexInClue" | "isRebusMode" | "cursorIndex" | "fromSavedState">

export type SavedGameState = Omit<GameStateToSave, "checked"> & {
    checked: [number, boolean][]
}

export class GameStateStore implements GameState {
    clue: undefined | Clue = $state(undefined)
    indexInClue: number = $state(-1)
    grid: string[] = $state([])
    checked = new SvelteMap<number,boolean>([])
    completion: CompletionState = $state("incomplete")
    elapsedTime = $state(0)
    isRebusMode: boolean = $state(false)
    
    get cursorIndex() {
        return this.clue?.indexes[this.indexInClue]
    }

    constructor(crossword?: Crossword) {
        if(crossword) {
            this.grid = crossword.grid.map((_) => "")
        }
    }

    static fromSavedState(savedState: GameStateToSave): GameStateStore {
        const newStore = new GameStateStore()

        newStore.checked = savedState.checked
        newStore.completion = savedState.completion
        newStore.grid = savedState.grid
        newStore.elapsedTime = savedState.elapsedTime

        return newStore
    }

}