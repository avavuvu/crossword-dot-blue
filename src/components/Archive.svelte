<script lang="ts">
    import { formatCrosswordDocument } from "$lib/formatCrossword";
    import type { GameStateToSave, SavedGameState } from "$lib/gameState.svelte";
    import { Navigation } from "$lib/navigation";
    import type { CompletionState, Crossword, CrosswordCollection, CrosswordDocument } from "$lib/types";
    import { Badge, BadgeCheck, BadgeHelp, CornerDownRight } from "lucide-svelte";
    import Loading from "./completion/Loading.svelte";
    import { StorageManager } from "$lib/storageManager";

    const { collection }: { collection: CrosswordCollection } = $props()

    type CrosswordAndCompletion = Crossword & {
        completion: CompletionState | null
    }

    type CrosswordArchiveDisplay = [
        string, 
        CrosswordAndCompletion[]
    ][]

    const fetchCrosswords = async (): Promise<CrosswordArchiveDisplay> => {
        try {
            const response = await fetch(`/api/crosswords/${collection}/archive`)
            if(!response.ok) {
                const error = await response.json();
                throw new Error(error.message);
            }
            const crosswordDocuments: CrosswordDocument[] = await response.json()

            const crosswordsByMonth: Map<string, CrosswordAndCompletion[]> = new Map()
            crosswordDocuments.forEach((doc) => {
                const crossword = formatCrosswordDocument(doc, collection)
                const year = crossword.metadata.date.getFullYear()
                const month = crossword.metadata.date.toLocaleString('default', { month: 'long' })

                const key = `${month} ${year}`

                if (!crosswordsByMonth.has(key)) {
                    crosswordsByMonth.set(key, []);
                }

                // storage checker
                let completion: CompletionState | null = null
                const storedCrosswordDocumentString = localStorage.getItem(`${collection}_${doc.documentId}`) as string | null

                if(storedCrosswordDocumentString) {
                    const storedCrosswordDocument: GameStateToSave = StorageManager.parse(storedCrosswordDocumentString)
                    completion = storedCrosswordDocument.completion 
                }
                
                crosswordsByMonth.get(key)?.push({
                    ...crossword,
                    completion
                })
            })

            return [...crosswordsByMonth]
        }
        catch(e) {
            throw e
        }
    }
</script>

<svlete:head>
    <title>
        {collection} Archive – Crossword Dot Blue
    </title>
</svlete:head>

{#snippet completionSquare(completionState: CompletionState | null)}
    {#if completionState === "won"}
        <span class="text-primary classic:text-secondary grove:text-selected">
            <BadgeCheck/>
        </span>
    {:else if completionState === null}
        <span class="text-selected grove:text-primary">
            <Badge/>
        </span>
    {:else}
        <span class="text-selected grove:text-primary">
            <BadgeHelp/>
        </span>
    {/if}
{/snippet}

<div class="mx-auto max-w-main min-h-[600px] midnight:text-black dark:text-black">
    <h1 class="font-bold text-4xl py-8 mx-4 lg:mx-12">{collection === "mini" ? "Mini" : "Big"} Crosswords Archive</h1>

    <div class="flex flex-wrap gap-y-8 justify-center">
        {#await fetchCrosswords()}
            <div class="w-[300px] mx-auto">
                <Loading/>

            </div>
        {:then months} 
            {#each months as [month, crosswords]}
                <div class="my-2 mx-4 flex flex-col w-[200px] md:lg:w-[300px] lg:w-[400px] border-2 border-primary rounded-xl overflow-clip max-w-96">
                    <div class="bg-primary text-white">
                        <h1 class="capitalize mx-4 font-bold">{month}</h1>

                    </div>
                    {#each crosswords as {completion, metadata: {title, date, difficulty, collection, documentId}, width, height}}
                        <a href={Navigation.construcrCrosswordUrl(collection, documentId)} class="flex flex-col mx-4">
                            <span class={[
                                "inline-flex justify-between",
                                completion === "won" ? "text-black/50" : ""
                                ]}>
                                <span class='font-bold inline-flex gap-2'>
                                    <span>
                                        {@render completionSquare(completion)}
                                    </span>
                                    {title || 
                                        date.toLocaleDateString('default', { 
                                            day: "2-digit", 
                                            month: "short", 
                                            weekday: "long"
                                        })}
                                </span>
                                {width}x{height}
                            </span>

                        </a>
                    {/each}
                </div>
            {/each}

        {/await}
    </div>
</div>