<script lang="ts">
    import { options } from '$lib/options.svelte';
    import { Info } from 'lucide-svelte';
    import { draw, fly } from 'svelte/transition';
    import Board from './board/Board.svelte';
    import Chars from './board/Chars.svelte';
    import Cursor from './board/Cursor.svelte';
    import OrderNumbers from './board/OrderNumbers.svelte';
    import WordBoundariesAndCircles from './board/WordBoundariesAndCircles.svelte';
    import ClueBanner from './clues/ClueBanner.svelte';
    import Clues from './clues/Clues.svelte';
    import MobileClues from './clues/MobileClues.svelte';
    import VictoryDrawer from './completion/VictoryDrawer.svelte';
    import MobileKeyboard from './input/MobileKeyboard.svelte';
    import type { Crossword } from '$lib/game/types';
    import { gameManager } from '$lib/game/gameManager.svelte';
    import AssistBar from './navbar/AssistBar.svelte';
    import Timer from './completion/Timer.svelte';
    import CrosswordIntro from './completion/CrosswordIntro.svelte';
    import Options from './options/Options.svelte';
    import Input from './input/Input.svelte';
    import { onMount } from 'svelte';

    const { crossword, showSettings = $bindable(false) }: { crossword: Crossword, showSettings: boolean } = $props()
    const { metadata } = crossword

    gameManager.init(crossword)

    const transition = {
        in: { x: 100, duration: 200, delay: 150 },
        out: { x: 100, duration: 200}
    }

    const startCrossword = () => {
        displayState = "game"
        gameManager.timeManager.start()
    }

    $effect(() => {
        hasWon = gameManager.state.completion === "won"
        drawerOpen = gameManager.state.completion !== "incomplete"
    })

    let hasWon = $state(false)
    let displayState: "intro" | "game" = $state(
        gameManager.state.completion === "won"
            ? "game"
            : "intro"
    )
    let drawerOpen = $state(false)

    onMount(() => {
        displayState = drawerOpen ? "game" : "intro"
    })
</script>

<Input {crossword}/>

{#snippet TopBarSnippet(rebusButton: boolean)}
    <div class="flex flex-row justify-evenly bg-primary text-white  mx-auto w-full  flex-none
                h-12 rounded-xl ">
        {#if hasWon}
            <button class="h-full aspect-square" onclick={() => drawerOpen = !drawerOpen}>
                <div class="hover:-translate-y-0.5 transition-transform h-full flex items-center justify-center 
                    hover:bg-white hover:text-black rounded-xl ">
                    <Info/>
                </div>
            </button>
        {:else}
            <AssistBar rebus={crossword.metadata.isRebus && rebusButton}/>
        {/if}
    </div>
{/snippet}

<div class="grid-1-1 grid text-black">
    {#if showSettings}
        <div class="grid-stack max-w-main mx-auto" in:fly={transition.in} out:fly={transition.out}>
            <Options/>
        </div>
    {:else}
        {#if drawerOpen}
            <div class="grid-stack max-w-main mx-auto" in:fly={transition.in} out:fly={transition.out}>
                <VictoryDrawer bind:drawerOpen/>
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
    {/if}
</div>

{#snippet game()}

<div class="flex justify-center pt-2">
    <Timer/>
</div>

<div class="h-screen grid grid-rows-[1fr_auto]">
    <div id="board-and-clues" class="grid min-h-0 p-2 pr-3 gap-2 min-w-0 max-w-main mx-auto w-full">
        {@render TopBarSnippet(true)}
        <div id="board" class="flex flex-col justify-normal items-center min-w-0 min-h-0 gap-2">
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
        </div>

    </div>

    <div id="mobile">
        <MobileClues/>
        <MobileKeyboard {crossword}/>
    </div>

</div>
{/snippet}