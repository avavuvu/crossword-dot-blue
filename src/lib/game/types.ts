import type { CollectionEntry } from "astro:content"
import type { CrosswordMetadata } from "src/content.config"
export interface Clue {
    indexes: number[]
    word: string
    hint: string
    isHorizontal: boolean
    id: string
}

export type CompletionState = "incomplete" | "completeButWrong" | "won"

export type Orientation = 'across' | 'down'

export interface Cell {
    solid: boolean
    circled: boolean
    gray: boolean
    wordBoundary: { [key in Orientation]?: boolean }

    clues: Partial<Record<Orientation, Clue>>
}

export type CharacterSet = "default" | "numbers" | "special"

export interface Crossword {
    metadata: CrosswordMetadata
    width: number
    height: number
    grid: Cell[]
    clues: Clue[]
    solution: string[]
    links: Clue[][],
}

export type ClueDocument = {
    coords: number[]
    word: string
    clue: string
    isHorizontal: boolean
    id: string
}

export type CrosswordCollection = "big" | "mini" | "secret"