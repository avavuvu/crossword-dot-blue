<script lang="ts">
    import { formatCrosswordDocument } from "$lib/game/formatCrossword";
    import type { GameStateToSave } from "$lib/game/gameState.svelte";
    import { Navigation } from "$lib/game/navigation";
    import type { CompletionState, Crossword, CrosswordCollection } from "$lib/game/types";
    import { Badge, BadgeCheck, BadgeHelp } from "lucide-svelte";
    import Loading from "./completion/Loading.svelte";
    import { StorageManager } from "$lib/game/storageManager";
    import type { CwFile } from "src/content.config";

    const { collection, crosswordsByMonth }: { 
        collection: CrosswordCollection,
        crosswordsByMonth: Map<string, CwFile[]>
    } = $props()

    const crosswordsArray: [string, (CwFile & { completion: CompletionState | null})[]][] = [...crosswordsByMonth]
        .reverse()
        .map(([month, crosswords]) => 
            [month, crosswords.map(doc => {
                // storage checker
                let completion: CompletionState | null = null
                const storedCrosswordDocumentString = localStorage.getItem(`${collection}_${doc.metadata.id}`) as string | null
            
                if(storedCrosswordDocumentString) {
                    const storedCrosswordDocument: GameStateToSave = StorageManager.parse(storedCrosswordDocumentString)
                    completion = storedCrosswordDocument.completion 
                }

                return {
                    completion,
                    ...doc
                }
            })])
        
    
</script>

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
        {#each crosswordsArray as [month, crosswords]}
            <div class="my-2 mx-4 flex flex-col w-[200px] md:lg:w-[300px] lg:w-[400px] border-2 border-primary rounded-xl overflow-clip max-w-96">
                <div class="bg-primary text-white">
                    <h1 class="capitalize mx-4 font-bold">{month}</h1>

                </div>
                {#each crosswords as {metadata: {name, date, id}, grid: {width, height}, completion}}
                    <a href={Navigation.construcrCrosswordUrl(collection, id)} class="flex flex-col mx-4">
                        <span class={[
                            "inline-flex justify-between",
                            completion === "won" ? "text-black/50" : ""
                            ]}>
                            <span class='font-bold inline-flex gap-2' style:font-style={name ? "italic" : ""}>
                                <span>
                                    {@render completionSquare(completion)}
                                </span>
                                {name || 
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
    </div>
</div>