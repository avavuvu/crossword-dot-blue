import { getOrientation, indexToCoords } from "./Coord"
import type { Cell, Clue, Crossword, CrosswordCollection} from "../game/types"
import type { CrosswordDocument } from "src/content.config"

export const formatCrosswordDocument = (crosswordDocument: CrosswordDocument, collection: CrosswordCollection): Crossword => {
    const clues: Clue[] = crosswordDocument.content.clues.map(clue => ({
        indexes: clue.coords,
        word: clue.word,
        hint: clue.clue,
        isHorizontal: clue.isHorizontal,
        id: clue.id,
    }))

    const grid: Cell[] = crosswordDocument.content.grid.map(char => {
        return {
            solid: char === ".",
            circled: char.toLowerCase() === char && char !== ".",
            gray: false, // i dont think i ever implemneted gray
            clues: {},
            wordBoundary: {}

        }
    })

    for (const clue of crosswordDocument.content.clues) {
        if(!clue.wordBoundaries) {
            continue
        }

        for(const boundary of clue.wordBoundaries) {
            const incrementor = clue.coords[0] + (clue.isHorizontal
                ? boundary - 1
                : (boundary - 1) * crosswordDocument.content.width);
            
            grid[incrementor].wordBoundary[
                getOrientation(clue.isHorizontal)
            ] = true
        }
    }

    clues.forEach((clue) => {
        clue.indexes.forEach(positionIndex => {
            grid[positionIndex].clues[clue.isHorizontal ? 'across' : 'down'] = clue

        })
    })

    grid.forEach((cell, index) => {
        if(cell.solid) { return }

        if(!cell.clues.across && !cell.clues.down) {
            // add a "dummy" clue for any island cells that doesn't have clues. this is rare but it does exist
            cell.clues.across = {
                indexes: [index],
                word: "",
                hint: "",
                isHorizontal: true,
                id: ""
            }
        }
    })

    const solution = crosswordDocument.content.grid.map(char => char !== "." ? char.toLowerCase() : "")

    const links = crosswordDocument.content.links?.map(linkGroup => 
        linkGroup.map(id => {
            const clue = clues.find(clue => id === clue.id)

            if(!clue) {
                console.error(`Unable to find ID: ${id} in link group!`)
            }

            return clue!
        })
    ) ?? []

    return { 
        metadata: {
            ...crosswordDocument.metadata,
            collection
        },
        ...crosswordDocument.content,
        grid, clues, solution, links,
        width: crosswordDocument.content.width,
        height: crosswordDocument.content.height,
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