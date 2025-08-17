<script lang="ts">
    import { CrosswordAPI } from "$lib/crosswordLoader";
    import { Navigation } from "$lib/game/navigation";
    import type { Cell, Crossword, CrosswordCollection } from "$lib/game/types";
    import MiniGridDisplay from "@/board/MiniGridDisplay.svelte";
    import Chillies from "@/completion/Chillies.svelte";
    import Navbar from "@/navbar/Navbar.svelte";
    import NavbarButton from "@/navbar/NavbarButton.svelte";
    import { wordLogo } from "@/svg/wordLogo.svelte";
    import { Settings } from "lucide-svelte";
    import { fade } from "svelte/transition";

    const getLatestPuzzles = () => {
        const latestBig = CrosswordAPI.loadCrossword("big")
        const latestMini = CrosswordAPI.loadCrossword("mini")

        return {latestBig, latestMini}
    }

    const crosswords = getLatestPuzzles()

    const loadingText = [..."LOADING"]
    const skeletonGridMini: (Cell & { char: string})[] = Array.from({length: 5*5}, (_,index) => ({
        solid: index % 3 === 0,
        circled: false,
        wordBoundary: {},
        gray: false,
        clues: {},
        char: loadingText[index % loadingText.length]
    }))
    const skeletonGridBig: (Cell & { char: string})[] = Array.from({length: 15*15}, (_,index) => ({
        solid: index % 4 === 0,
        circled: false,
        wordBoundary: {},
        gray: false,
        clues: {},
        char: loadingText[index % loadingText.length]
    }))
</script>

<title>Crossword Dot Blue</title>

<Navbar>
    {#snippet optionsSnippet()}
        <NavbarButton props={{}}>
            <a href="/options" class="w-full h-full flex justify-center items-center">
                <h1><Settings/></h1>
            </a>
        </NavbarButton>
    {/snippet}
</Navbar>

<div class="midnight:text-black dark:text-black">


    <div class="max-w-main mx-auto ">
        <div class="max-w-[700px] mx-auto p-8">
            {@render wordLogo()}
        </div>
        <p class="text-center text-xl italic">
            The free weekly* crossword
        </p>
    </div>

    {#snippet card(collection: string, skeletonGrid: (Cell & {char?: string})[], size: number, getCrossword: Promise<Crossword>)}
        <div class="w-[400px] mx-4">
            <a class="text-white text-4xl" href="/">
                <h2>
                    <span>The</span>
                    <span class="font-bold">{collection}</span>
                </h2>
            </a>

            <div class="grid rounded-b-2xl overflow-clip">
                <div class="row-start-1 row-end-2 col-start-1 col-end-2">
                    <MiniGridDisplay width={size} height={size} grid={skeletonGrid} />
                    <div class="bg-white min-h-32">
    
                    </div>

                </div>
                {#await getCrossword}
                    <!--  -->
                {:then {width, height, grid, metadata}} 
                    {@const href = Navigation.construcrCrosswordUrl(metadata.collection, metadata.documentId)}
                    <div transition:fade class="row-start-1 row-end-2 col-start-1 col-end-2">
                        <a {href} class="border-2 border-white block">
                            <MiniGridDisplay {width} {height} {grid} />
                        </a>
                        <div class="bg-white text-center min-h-32">
                            <a {href} class="text-xl">
                                {#if metadata.title !== ""}
                                    {metadata.title}
                                {:else}
                                    {metadata.collection} Crossword
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
                {/await}
            </div>

        </div>
    {/snippet}

    <div class="bg-primary w-full  flex flex-wrap gap-4 justify-center mb-24 mt-12 py-8">

        {@render card("Mini", skeletonGridMini, 5, crosswords.latestMini)}
        {@render card("Big", skeletonGridBig, 15, crosswords.latestBig)}
    </div>

    <div class="flex flex-wrap gap-4 justify-center text-4xl underline">
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
    </div>
</div>