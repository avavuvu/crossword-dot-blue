import type { CrosswordCollection } from "./types";

export class Navigation {
    static construcrCrosswordUrl = (collection: CrosswordCollection, id: string) => {
        return `/${collection}/${id}`
    }
}