<script lang="ts">
    import { Navigation } from "$lib/game/navigation";
    import type { CrosswordCollection } from "$lib/game/types";
    import MiniGridDisplay from "@/board/MiniGridDisplay.svelte";
    import Chillies from "@/completion/Chillies.svelte";
    import { wordLogo } from "@/svg/wordLogo.svelte";
  import { Settings, Smile } from "lucide-svelte";
    import type { CrosswordDocument } from "src/content.config";
    import type { Snippet } from "svelte";
    import { fade } from "svelte/transition";

    const { latestBig, latestMini, children }: { latestBig: CrosswordDocument, latestMini: CrosswordDocument, children: Snippet } = $props()

</script>


<div class="midnight:text-black dark:text-black">

    <div class="max-w-main mx-auto ">
        <div class="max-w-[700px] mx-auto p-8">
            {@render wordLogo()}
        </div>
        <p class="text-center text-xl italic">
            The free weekly* crossword
        </p>
    </div>

    {#snippet card(collection: string, {metadata, content: { width, height, grid} }: CrosswordDocument)}
        {@const href = Navigation.construcrCrosswordUrl(collection.toLowerCase() as CrosswordCollection, metadata.id)}
        <div class="w-[400px] mx-4">
            <a class="text-white text-4xl" href="/">
                <h2>
                    <span>The</span>
                    <span class="font-bold">{collection}</span>
                </h2>
            </a>

            <div class="grid rounded-b-2xl overflow-clip">
                
                <div transition:fade class="row-start-1 row-end-2 col-start-1 col-end-2">
                    <a {href} class="border-2 border-white block">
                        <MiniGridDisplay {width} {height} {grid} />
                    </a>
                    <div class="bg-white text-center min-h-32">
                        <a {href} class="text-xl">
                            {#if metadata.name !== ""}
                                {metadata.name}
                            {:else}
                                {collection} Crossword
                            {/if}
                        </a>
                        <p class="text-xl">
                            <span class="font-bold">{metadata.date.toDateString()}</span> — by {metadata.author}
                        </p>
                        <hr class="border-t-primary border-t-2">
                        <p class="py-2">
                            <Chillies amount={metadata.difficulty}/>
                        </p>
                    </div>    
                </div>
            </div>

        </div>
    {/snippet}

    <div class="bg-primary w-full  flex flex-wrap gap-4 justify-center mb-24 mt-12 py-8">

        {@render card("Mini", latestMini)}
        {@render card("Big", latestBig)}
    </div>

    <div class="grid grid-cols-2 max-w-[600px] mx-auto gap-4 justify-center text-4xl underline">
        <a href="/mini/archive" class="text-primary border-2 border-primary p-4 rounded-2xl">
            <h2>
                Play More <span class="font-bold">Mini</span> Puzzles
            </h2>
        </a>
        <a href="/big/archive" class="text-primary border-2 border-primary p-4 rounded-2xl">
            <h2>
                Play More <span class="font-bold">Big</span> Puzzles
            </h2>
        </a>
        <a href="/options" class="text-primary border-2 border-primary p-4 rounded-2xl inline-flex gap-2">
            <Settings size={48}></Settings>
            <h2>
                Settings 
            </h2>
        </a>
        <a href="/about" class="text-primary border-2 border-primary p-4 rounded-2xl inline-flex gap-2">
            <Smile size={48}/>
            <h2>
                About 
            </h2>
        </a>
    </div>

</div>