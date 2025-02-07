import type { isFileLoadingAllowed } from "vite"
import { getOrientation, indexToCoords } from "./Coord"
import type { Cell, CharacterSet, Clue, Crossword, CrosswordCollection, CrosswordDocument, CrosswordMetadata } from "./types"

export const formatCrosswordDocument = (crosswordDocument: CrosswordDocument, collection: CrosswordCollection): Crossword => {
    const clues: Clue[] = crosswordDocument.clues.map(clue => ({
        indexes: clue.coords,
        word: clue.word,
        hint: clue.clue,
        isHorizontal: clue.isHorizontal,
        id: clue.id,
    }))

    let characterSet: CharacterSet = "default"
    const numberPattern = /\d+/g
    const specialPattern = /[^A-Za-z0-9]+/g

    let rebusEnabled = false

    const grid: Cell[] = crosswordDocument.grid.map(char => {
        
        if(char !== ".") {
            if(numberPattern.test(char)) {
                characterSet = "numbers"
            }
    
            if(specialPattern.test(char)) {
                characterSet = "special"
            }
            
            if(char.length > 1) {
                rebusEnabled = true
            }

        }

        return {
            solid: char === ".",
            circled: char.toLowerCase() === char && char !== ".",
            gray: false,
            clues: {},
            wordBoundary: {}

        }
    })

    clues.forEach((clue) => {
        clue.indexes.forEach(positionIndex => {
            grid[positionIndex].clues[clue.isHorizontal ? 'across' : 'down'] = clue

            if(calculateWordBoundaries(clue, positionIndex, crosswordDocument.width )) {
                grid[positionIndex].wordBoundary[
                    getOrientation(clue.isHorizontal)
                ] = true
            }
            
        })
    })

    grid.forEach((cell, index) => {
        if(cell.solid) { return }

        if(!cell.clues.across && !cell.clues.down) {
            // add a "dummy" clue for any island cells that dont have clues. this is rare but it does exist
            cell.clues.across = {
                indexes: [index],
                word: "",
                hint: "",
                isHorizontal: true,
                id: ""
            }
        }
    })

    const completedGrid = crosswordDocument.grid.map(char => char !== "." ? char.toLowerCase() : "")

    let links: Clue[][] = []
    if(crosswordDocument.links && crosswordDocument.links?.length > 0) {
        links = crosswordDocument.links.map(({ids}) => {
            const linkGroup = (ids as string[]).map((id) => {
                const clue = clues.find((clue) => id === clue.id)
    
                if(!clue) {
                    console.error("Unable to find ID in link group!")
                }
    
                return clue!
            })
    
            return linkGroup
        })
    }

    const date = new Date(0)
    date.setUTCSeconds(crosswordDocument.date.seconds)

    const metadata: CrosswordMetadata = {
        documentId: crosswordDocument.documentId,
        collection: collection,
        author: crosswordDocument.author,
        title: crosswordDocument.name,
        difficulty: crosswordDocument.difficulty,
        date
    }

    return { 
        grid, clues, completedGrid, links, metadata, characterSet, 
        rebus: rebusEnabled,
        width: crosswordDocument.width,
        height: crosswordDocument.height,
    }
}

// TODO: in an ideal world this wouldn't exist, and it would be calculate when uploaded.
const calculateWordBoundaries = (clue: Clue, cellIndex: number, width: number) => {
    let clueWithSeperator = [...clue.word]

    let hasWordBoundary = false

    if(clueWithSeperator.includes("|")) {
        const seperatorIndices: number[] = []
        while(clueWithSeperator.indexOf("|") !== -1) {
            const index = clueWithSeperator.indexOf("|")
            clueWithSeperator.splice(index, 1)
            seperatorIndices.push(index)
        }

        for(const seperatorIndex of seperatorIndices) {
            if(!clue.isHorizontal === clue.isHorizontal) {
                continue
            }

            if(clue.isHorizontal) {
                if(clue.indexes[0] + seperatorIndex - 1 === cellIndex) {
                    hasWordBoundary = true
                } 
            } else {
                if(clue.indexes[0] + ((seperatorIndex - 1) * width) === cellIndex) {
                    hasWordBoundary = true
                } 
            }
        }
    }

    return hasWordBoundary
}


export const orderClues = (clues: Clue[], settings: "ordinal" | "spatial", width?: number) => {
    let orderedClues
    
    switch(settings) {
        case "ordinal":
            orderedClues = clues.toSorted((clueA, clueB) => +clueA.isHorizontal - +clueB.isHorizontal)
            break;
        case "spatial":
            orderedClues = clues.toSorted((clueA, clueB) => {
                const clueACoord = indexToCoords(clueA.indexes[0], width!)
                const clueBCoord = indexToCoords(clueB.indexes[0], width!)
    
                return clueACoord[1] - clueBCoord[1] || clueACoord[0] - clueBCoord[0]
            })
            break;
    }

    return orderedClues
}