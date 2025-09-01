import { FirestoreApp } from '$lib/firestore/firestore'
import { config } from '$lib/firestore/firestore.config'
import { formatCrosswordDocument } from '$lib/game/formatCrossword'
import { date } from 'astro:schema'
import { initializeApp, type FirebaseApp } from 'firebase/app'
import { collection, getDocs, getFirestore, orderBy, query, QueryDocumentSnapshot, QuerySnapshot, Timestamp, where, type Firestore } from 'firebase/firestore/lite'
import { mkdir, writeFile } from 'node:fs/promises'

const crosswordCollection = "crosswords"

// const firestore = await FirestoreApp.create()
// const searchQuery = query(
//     collection(firestore.db, crosswordCollection),
// )

// const snapshot = await getDocs(searchQuery)
// const crosswords = firestore.convertSnapshotToCrosswordDocument(snapshot)

const REBUS_REPLACEMENT = '\u{E000}';

const calculateWordBoundaries = (clue: string) => {
    const rebusRegex = /(\(+.*\))/g
    const replaced = clue.replaceAll(rebusRegex, REBUS_REPLACEMENT)

    console.log(replaced)

    const regex = /\s/g
    const iterator = replaced.trim().matchAll(regex)
    
    return [...iterator].map(
        (match, index) => match.index - index
    )
}

const saveCrossword = async (crosswordDocument: CrosswordDocument) => {
    const { name, documentId, date: timestamp, width, height, links, clues, grid, difficulty} = crosswordDocument
    
    const date = timestamp as Timestamp

    let crosswordFormatVersion2 = {
        metadata: {
            author: "Ava Vu",
            date: date.toDate() ,
            name,
            id: documentId,
            difficulty,
        },
        content: {
            width,
            height,
            grid,
            links: links?.map(({ids}) => ids.map(id => id)),
            clues: clues.map((clue) => ({
                ...clue,
                wordBoundaries: calculateWordBoundaries(clue.word),
                word: clue.word.replaceAll("|", "")
            })),
        }
    }

    await mkdir(`./scripts/output/${crosswordCollection}/`, {
        recursive: true
    })

    await writeFile(`./scripts/output/${crosswordCollection}/${documentId}.json`, JSON.stringify(crosswordFormatVersion2))
}

// crosswords.forEach(crossword => saveCrossword(crossword))