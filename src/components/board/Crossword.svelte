<script lang="ts">
    import type { Crossword } from "$lib/game/types";
    import Cursor from "@/board/Cursor.svelte";
    import Chars from "@/board/Chars.svelte";
    import OrderNumbers from "@/board/OrderNumbers.svelte";
    import Clues from "@/clues/Clues.svelte";
    import WordBoundaries from "@/board/WordBoundariesAndCircles.svelte";
    import Board from "@/board/Board.svelte";
    import ClueBanner from "@/clues/ClueBanner.svelte";
    import AssistBar from "@/navbar/AssistBar.svelte";
    import { Collapsible } from "melt/builders";
    import { fly } from "svelte/transition";
    import { gameManager } from "$lib/game/gameManager.svelte";
    import VictoryDrawer from "@/completion/VictoryDrawer.svelte";
    import { MediaQuery } from "svelte/reactivity";
    import Timer from "@/completion/Timer.svelte";
    import MobileKeyboard from "@/input/MobileKeyboard.svelte";
    import { options } from "$lib/options.svelte";
    import MobileClues from "@/clues/MobileClues.svelte";
    import { Info } from "lucide-svelte";
    import { UAParser } from "ua-parser-js"

    const { crossword }: { crossword: Crossword } = $props()
    const { metadata } = crossword

    const isLargeScreen = new MediaQuery('min-width: 900px');

    const victoryDrawer = new Collapsible()

    $effect(() => {
        victoryDrawer.open = gameManager.state.completion !== "incomplete"
        hasWon = gameManager.state.completion === "won"
    })

    let hasWon = $state(false)

    const { device } = UAParser(navigator.userAgent)
    const isLandscapeTablet = $derived(device.type === "tablet" && isLargeScreen.current)
    const showKeyboard = $derived(isLandscapeTablet || options.ForceKeyboard.setting || !isLargeScreen.current)

</script>

{#snippet TopBarSnippet(rebusButton: boolean)}
    <div class="flex flex-row justify-evenly bg-primary text-white  mx-auto max-w-[500px] w-full 
                h-12 rounded-xl ">
        {#if hasWon}
            <button class="h-full  min-w-8 mx-auto aspect-square" onclick={() => victoryDrawer.open = !victoryDrawer.open}>
                <div class="hover:-translate-y-0.5 transition-transform h-full flex items-center justify-center 
                    hover:bg-white hover:text-black  rounded-xl ">
                    <Info/>
                </div>
            </button>
        {:else}
            <AssistBar rebus={crossword.metadata.isRebus && rebusButton}/>
        {/if}
    </div>
{/snippet}

<div>
    <div class={[
        "mx-auto max-w-main text-center flex justify-center items-center pt-2",
        "flex-col lg:flex-row lg:gap-8"
        ]}>
        <span>
            {#if metadata.name} 
            <span class="italic">
                {metadata.name}
            </span> 
            {:else}
            <span class="italic">
                {crossword.width} x {crossword.height} Crossword
            </span>
            {/if}
            – By {metadata.author}
        </span>
        
        {#if options.ShowTimer.setting}
            <Timer/>
        {/if}
    </div>
    
    <div class={showKeyboard ? "flex flex-col h-full" : ""}>
        <div class={[
            "flex gap-2 max-w-main w-full mx-auto",
            showKeyboard ? "" : " h-[100dvh]",
        ]}>
            <div class="mx-auto flex-1 min-w-[50%]" {...victoryDrawer.root} >
                <div class="flex justify-between flex-col h-full gap-1">
                    {#if !isLargeScreen.current}
                        {@render TopBarSnippet(false)}
                    {/if}
                    {#if victoryDrawer.open && !isLargeScreen.current}
                        <div class="" {...victoryDrawer.content} 
                            in:fly={{ x: 100, duration: 200, delay: 150 }}
                            out:fly={{ x: 100, duration: 200}}>

                            <VictoryDrawer bind:drawerOpen={victoryDrawer.open}/>
                        </div>
                    {:else}
                        <svg
                            class="border-2 border-primary rounded-2xl mx-auto [@media(max-width:500px)]:mx-4 lg:mx-auto"
                            viewBox="0 0 {crossword.width} {crossword.height}">
                            <Board crossword={crossword}/>
                            <Cursor {crossword}/>
                            <Chars {crossword}/>
                            <OrderNumbers {crossword}/>
                            <WordBoundaries {crossword}/>
                        </svg>
        
                        <div>
                            {#if isLargeScreen.current}
                                {#if !showKeyboard}
                                    <ClueBanner { crossword }/>
                                {/if}
                            {/if}
                        </div>

                    {/if}
                </div>
            </div>
    
            {#if isLargeScreen.current}
                <div class="h-full w-full">
                    <div class="flex flex-row justify-evenly bg-primary text-white  mx-auto max-w-main w-full 
                    h-8 lg:h-12 rounded-xl">
                        {@render TopBarSnippet(true)}
                    </div>
                    <div {...victoryDrawer.root} class="grid grid-rows-1 grid-cols-1 h-full w-full">
                        {#if victoryDrawer.open}
                            <div class="w-full col-start-1 col-end-2 row-start-1 row-end-2" {...victoryDrawer.content} 
                                in:fly={{ x: 100, duration: 200, delay: 150 }}
                                out:fly={{ x: 100, duration: 200}}>
    
                                <VictoryDrawer bind:drawerOpen={victoryDrawer.open}/>
                            </div>
                        {:else}
                            <div class={["col-start-1 col-end-2 row-start-1 row-end-2 w-full h-full"]}
                                in:fly={{ x: 100, duration: 200, delay: 150 }}
                                out:fly={{ x: 100, duration: 200}}>
                                
                                <Clues {crossword}/>
                            </div>
                        {/if}
                    </div>
                </div>  
            {/if}
            
        </div>
        
        {#if showKeyboard}
            <MobileClues />
            <MobileKeyboard {crossword}/>
        {/if}
    </div>

    
</div>