import { json, error } from '@sveltejs/kit'
import { firestoreApp } from '$lib/firestore/firestore'

export const GET = async ({ params }) => {
    try {
        const puzzle = await firestoreApp.getPuzzleFromId(params.id, "mini")

        return json(puzzle)

    } catch (e) {
        throw error(404, (e as Error).message)
    }
}

