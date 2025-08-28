import { getOrientation, indexToCoords } from "./Coord"
import type { Cell, Clue, Crossword, CrosswordCollection} from "../game/types"
import type { CwFile } from "src/content.config"
import { calculateWordBoundaries } from "./wordBoundaries"

export const formatCrosswordDocument = (crosswordDocument: CwFile, collection: CrosswordCollection): Crossword => {

    const clues: Clue[] = crosswordDocument.clues.map(clue => ({
        indexes: clue.coords,
        word: clue.word,
        hint: clue.hint.text,
        // TODO: Support multi hint clues
        isHorizontal: clue.isHorizontal,
        id: clue.id,
    }))

    const grid: Cell[] = crosswordDocument.grid.cells.map(cell => {

        let circled = false

        if(cell.type === "text") {
            circled = cell.style?.circled || false
        }

        return {
            solid: cell.type === "block",
            circled: circled,
            gray: false, // i dont think i ever implemneted gray
            clues: {},
            wordBoundary: {}

        }
    })

    for (const clue of crosswordDocument.clues) {
        const wordBoundaries = calculateWordBoundaries(clue.word)
        
        if(!wordBoundaries) {
            continue
        }

        for(const boundary of wordBoundaries) {
            const incrementor = clue.coords[0] + (clue.isHorizontal
                ? boundary - 1
                : (boundary - 1) * crosswordDocument.grid.width);
            
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

    const solution = crosswordDocument.grid.cells.map(cell => cell.type === "text" ? cell.text.toLowerCase() : "")

    const links = crosswordDocument.links?.map(linkGroup => 
        linkGroup.map(id => {
            const clue = clues.find(clue => id === clue.id)

            if(!clue) {
                console.error(`Unable to find ID: ${id} in link group!`)
            }

            return clue!
        })
    ) ?? []

    return { 
        ...crosswordDocument,
        metadata: {
            ...crosswordDocument.metadata,
            date: new Date(crosswordDocument.metadata.date),
            collection
        },
        grid, clues, solution, links,
        width: crosswordDocument.grid.width,
        height: crosswordDocument.grid.height,
    }
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