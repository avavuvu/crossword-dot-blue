import { FormatDate } from "./date";
import { formatCrosswordDocument } from "./formatCrossword";
import type { Crossword, CrosswordCollection, CrosswordDocument } from "./types";

export class CrosswordAPI {
    

    static loadCrossword = async (collection: CrosswordCollection, id?: string): Promise<Crossword> => {
        try {
            let url = `/api/crosswords/${collection}`

            if(id) {
                url += `/${id}`
            }
            
            const response = await fetch(url)
            if(!response.ok) {
                const error = await response.json();
                throw new Error(error.message);
            }
        
            const crosswordDocument: CrosswordDocument = await response.json()
            const crossword = formatCrosswordDocument(crosswordDocument, collection)
        
            return crossword
        } catch(e) {
            throw e
        }
    }
}