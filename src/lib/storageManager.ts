import { SvelteMap } from "svelte/reactivity"
import type { GameStateToSave, SavedGameState } from "./gameState.svelte"
import type { CrosswordCollection } from "./types"

export interface StorageManagerInterface {
    get(): GameStateToSave | null
    set(gameState: GameStateToSave): void
}

export class StorageManager implements StorageManagerInterface {
    id: string

    constructor(collection: CrosswordCollection, documentId: string) {
        this.id = `${collection}_${documentId}`
    }

    stringify({grid, checked, completion, elapsedTime}: GameStateToSave) {
       const checkedArray = [...checked]
       const savedState: SavedGameState = {
            grid, 
            checked: checkedArray, 
            completion, 
            elapsedTime
        }

        return JSON.stringify(savedState)
    }

    parse(data: string): GameStateToSave {
        const parsed: SavedGameState = JSON.parse(data)
        const checkedMap = new SvelteMap<number, boolean>(parsed.checked)

        return {
            ...parsed,
            checked: checkedMap
        }
    }
    
    get(): GameStateToSave | null {
        const item = localStorage.getItem(this.id);
        return item ? this.parse(item) : null;
    }

    set(gameState: GameStateToSave) {
        const stringified = this.stringify(gameState)
        localStorage.setItem(this.id, stringified)
    }
}