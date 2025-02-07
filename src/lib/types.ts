export interface CrosswordMetadata {
    author: string,
    title: string,
    difficulty: number,
    date: Date,
    documentId: string,
    collection: CrosswordCollection
}

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
    completedGrid: string[]
    links: Clue[][],
    characterSet: CharacterSet
    rebus: boolean
}

export type ClueDocument = {
    coords: number[]
    word: string
    clue: string
    isHorizontal: boolean
    id: string
}

export type CrosswordCollection = "big" | "mini" | "secret"

export type CrosswordDocument = {
    documentId: string
    width: number
    height: number
    grid: string[]
    clues: ClueDocument[]
    name: string
    author: string
    difficulty: number
    date: { seconds: number }
    links?: {ids: string[]}[],
    collection: CrosswordCollection
}