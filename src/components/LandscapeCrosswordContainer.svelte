<script lang="ts">
    import { gameManager } from "$lib/game/gameManager.svelte";
    import type { Crossword } from "$lib/game/types";
    import { Info } from "lucide-svelte";
    import Board from "./board/Board.svelte";
    import Chars from "./board/Chars.svelte";
    import Cursor from "./board/Cursor.svelte";
    import OrderNumbers from "./board/OrderNumbers.svelte";
    import WordBoundariesAndCircles from "./board/WordBoundariesAndCircles.svelte";
    import Clues from "./clues/Clues.svelte";
    import MobileClues from "./clues/MobileClues.svelte";
    import MobileKeyboard from "./input/MobileKeyboard.svelte";
    import AssistBar from "./navbar/AssistBar.svelte";
    import Timer from "./completion/Timer.svelte";
    import { UAParser } from "ua-parser-js";
    import { MediaQuery } from "svelte/reactivity";
    import { options } from "$lib/options.svelte";
    import ClueBanner from "./clues/ClueBanner.svelte";
    import VictoryDrawer from "./completion/VictoryDrawer.svelte";
    import Input from "./input/Input.svelte";
    import { fly } from "svelte/transition";
    import CrosswordIntro from "./completion/CrosswordIntro.svelte";
    import Options from "./options/Options.svelte";

    const { crossword, showSettings = $bindable(false) }: { crossword: Crossword, showSettings: boolean } = $props()
    const { metadata } = crossword

    const transition = {
        in: { x: 100, duration: 200, delay: 150 },
        out: { x: 100, duration: 200}
    }

    gameManager.init(crossword)

    const startCrossword = () => {
        displayState = "game"
        gameManager.timeManager.start()
    }

    $effect(() => {
        halfScreenDrawer = gameManager.state.completion !== "incomplete"
        hasWon = gameManager.state.completion === "won"
    })

    let hasWon = $state(false)
    let halfScreenDrawer = $state(gameManager.state.completion !== "incomplete")
    let displayState: "intro" | "game" = $state(
        gameManager.state.completion === "won"
            ? "game"
            : "intro"
    )

    const { device } = UAParser(navigator.userAgent)
    const isTablet = $derived(device.type === "tablet")
    const showKeyboard = $derived(isTablet || options.ForceKeyboard.setting)

</script>

<Input {crossword}/>

{#snippet TopBarSnippet(rebusButton: boolean)}
    <div class="flex flex-row justify-evenly bg-primary text-white  mx-auto w-full  flex-none
                h-12 rounded-xl ">
        {#if hasWon}
            <button class="h-full  min-w-8 mx-auto aspect-square" onclick={() => halfScreenDrawer = !halfScreenDrawer}>
                <div class="hover:-translate-y-0.5 transition-transform h-full flex items-center justify-center 
                    hover:bg-white hover:text-black  rounded-xl ">
                    <Info/>
                </div>
            </button>
        {:else}
            <AssistBar rebus={crossword.rebus && rebusButton}/>
        {/if}
    </div>
{/snippet}


<div class="grid-1-1 grid text-black">
    {#if showSettings}
        <div class="grid-stack max-w-main mx-auto" in:fly={transition.in} out:fly={transition.out}>
            <Options/>
        </div>
    {:else}
        {#if displayState === "intro"}
            <div class="grid-stack max-w-main mx-auto" in:fly={transition.in} out:fly={transition.out}>
                <CrosswordIntro {crossword} {startCrossword}/>
            </div>
        {:else}
            <div class="grid-stack" in:fly={transition.in} out:fly={transition.out}>
                {@render game()}
            </div>
        {/if}
    {/if}
</div>

{#snippet game()}
<div class="h-screen grid grid-rows-[1fr_auto]">
    <div id="board-and-clues" class="grid grid-cols-[minmax(0,1fr)_minmax(0,1fr)] min-h-0 p-2 gap-2 min-w-0 max-w-main mx-auto w-full">
        <div id="board" class="flex flex-col justify-normal items-center min-w-0 min-h-0 gap-2">
            <div class="h-8 w-full text-right">
                <span>
                    {#if metadata.title} 
                    <span class="italic">
                        {metadata.title}
                    </span> 
                    {:else}
                    <span class="italic">
                        {crossword.width} x {crossword.height} Crossword
                    </span>
                    {/if}
                    {#if options.ShowTimer.setting}
                        – By {metadata.author}  
                    {/if}
                </span>
            </div>
            <svg
                class="border-2 border-block rounded-2xl"
                preserveAspectRatio="xMidYMid meet"
                viewBox="0 0 {crossword.width} {crossword.height}">
                <Board crossword={crossword}/>
                <Cursor {crossword}/>
                <Chars {crossword}/>
                <OrderNumbers {crossword}/>
                <WordBoundariesAndCircles {crossword}/>
            </svg>
            {#if !showKeyboard}
                <ClueBanner {crossword}/>
            {/if}
        </div>
        
        <div class="min-h-0 min-w-0 grid grid-rows-[2rem_3rem_auto] gap-2">
            <div class="h-8 w-full flex items-start ">
                {#if options.ShowTimer.setting}
                    <Timer/>
                {:else}
                    By {metadata.author}
                {/if}

            </div>
            
            {@render TopBarSnippet(crossword.rebus)}
            <div class="min-h-0 grid"  >
                <div class="grid grid-1-1 min-h-0">
                    {#if halfScreenDrawer}
                        <div class="grid-stack min-h-0 grid" in:fly={transition.in} out:fly={transition.out}>
                            <VictoryDrawer bind:drawerOpen={halfScreenDrawer}/>
                        </div>
                    {:else}
                        <div class="grid-stack min-h-0 grid  grid-cols-2" in:fly={transition.in} out:fly={transition.out}>
                            <Clues {crossword}/>
                        </div>
                    {/if}

                </div>
            </div>

        </div>
    </div>

    {#if showKeyboard}
        <div id="mobile">
            <MobileClues/>
            <MobileKeyboard {crossword}/>
        </div>
    {:else}
        <div class="h-8">
            
        </div>
    {/if}
</div>
{/snippet}