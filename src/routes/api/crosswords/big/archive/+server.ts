import { json, error } from '@sveltejs/kit'
import { firestoreApp } from '$lib/firestore/firestore'

export const GET = async () => {
    try {
        const puzzle = await firestoreApp.getAllPuzzles(new Date(), "big")

        return json(puzzle)

    } catch (e) {
        throw error(500, (e as Error).message)
    }
}
