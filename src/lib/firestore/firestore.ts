import { initializeApp, type FirebaseApp } from 'firebase/app'
import { collection, doc, DocumentSnapshot, getDoc, getDocs, getFirestore, limit, orderBy, query, QueryDocumentSnapshot, QuerySnapshot, Timestamp, where, type DocumentData, type Firestore } from 'firebase/firestore/lite'
import {config} from "./firestore.config"
import type { CrosswordCollection, CrosswordDocument } from '$lib/game/types'

export class FirestoreApp {
    firebase!: FirebaseApp
    db!: Firestore

    static create = async () => {
        const app = new FirestoreApp()
        app.firebase = initializeApp(config)
        app.db = getFirestore(app.firebase)

        return app
    }

    #puzzles = new Map<string, CrosswordDocument>()
    
    add(collection: CrosswordCollection, crossword: CrosswordDocument) {
        this.#puzzles.set(`${collection}_${crossword.documentId}`, crossword)
    }

    get(collection: CrosswordCollection, id: string): CrosswordDocument | undefined {
        return this.#puzzles.get(`${collection}_${id}`)
    }

    getAllPuzzles = async (date: Date, collectionName: string) => {
        const searchQuery = query(
            collection(this.db, collectionName),
            where("date", "<=", date),
            orderBy("date", "desc")
        )

        const snapshot = await getDocs(searchQuery)
        if(snapshot.empty) {
            return Promise.reject(new Error("Cannot find puzzles"))
        }

        return this.convertSnapshotToCrosswordDocument(snapshot)
    }

    getPuzzleFromId = async (id: string, collectionName: CrosswordCollection) => {
        try {
            const cachedPuzzle = this.get(collectionName, id)
            if(cachedPuzzle) {
                return cachedPuzzle
            }

            const snapshot = await getDoc(doc(this.db, `${collectionName}/${id}`));
            
            if (!snapshot.exists()) {
                throw new Error(`Document not found: ${collectionName}/${id}`);
            }
            
            return this.convertDocumentToCrosswordDocument(snapshot);
        } catch (error) {
            if (error instanceof Error) {
                throw error;
            }

            throw new Error('Failed to fetch puzzle');
        }
    }

    getLatestPuzzle = async (date: Date, collectionName: CrosswordCollection): Promise<CrosswordDocument> => {
        const dateQuery = query(
            collection(this.db, collectionName),
            where("date", "<=", date),
            orderBy("date", "desc"),
            limit(1)
        )
    
        const querySnapshot = await getDocs(dateQuery)
        if(querySnapshot.empty) {
            return Promise.reject(new Error("Cannot find puzzle at date"))
        }

        const crosswordDocument = this.convertSnapshotToCrosswordDocument(querySnapshot)[0]
        this.add(collectionName, crosswordDocument)

        return crosswordDocument
    }

    convertDocumentToCrosswordDocument = (querySnapshot: QueryDocumentSnapshot): CrosswordDocument => {
        return {
            ...querySnapshot.data(),
            documentId: querySnapshot.id
        } as CrosswordDocument
    }

    convertSnapshotToCrosswordDocument = (querySnapshot: QuerySnapshot): CrosswordDocument[] => {
        return querySnapshot.docs.map(this.convertDocumentToCrosswordDocument)
    }
}

export const firestoreApp = await FirestoreApp.create()
