<script lang="ts">
    import { formatCrosswordDocument } from "$lib/formatCrossword";
    import { Navigation } from "$lib/navigation";
    import type { Crossword, CrosswordCollection, CrosswordDocument } from "$lib/types";
    import Loading from "./completion/Loading.svelte";

    const { collection }: { collection: CrosswordCollection } = $props()

    const fetchCrosswords = async () => {
        try {
            const response = await fetch(`/api/crosswords/${collection}/archive`)
            if(!response.ok) {
                const error = await response.json();
                throw new Error(error.message);
            }
            const crosswordDocuments: CrosswordDocument[] = await response.json()

            const crosswordsByMonth: Map<string, Crossword[]> = new Map()
            crosswordDocuments.forEach((doc) => {
                const crossword = formatCrosswordDocument(doc, collection)
                const year = crossword.metadata.date.getFullYear()
                const month = crossword.metadata.date.toLocaleString('default', { month: 'long' })

                const key = `${month} ${year}`

                if (!crosswordsByMonth.has(key)) {
                    crosswordsByMonth.set(key, []);
                }
                
                crosswordsByMonth.get(key)?.push(crossword)
            })

            return [...crosswordsByMonth]
        }
        catch(e) {
            throw e
        }
    }
</script>

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
                    {#each crosswords as {metadata: {title, date, difficulty, collection, documentId}, width, height}}
                        <a href={Navigation.construcrCrosswordUrl(collection, documentId)} class="flex flex-col mx-4">
                            <span class="inline-flex justify-between">
                                <span class='font-bold'>
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