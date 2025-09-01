<script lang="ts">
    import { gameManager } from "$lib/game/gameManager.svelte";
    import type { Crossword } from "$lib/game/types";

    const { crossword }: { crossword: Crossword } = $props()

    const currentClueState = $derived(
        gameManager.state.clue?.indexes
            .flatMap((clueIndex) => [...gameManager.state.grid[clueIndex]])
            .map(char => char.toUpperCase()) ?? [])

    const maxLength = crossword.clues.reduce(
        (accumulator, clue) => Math.max(clue.word.length, accumulator), 0)

    const getFontSize = (char: string) => {
        if(char.length === 1) {
            return `0.05em`
        }
        
        if(char.length === 0) { 
            return "" 
        }

        return `${0.05 / char.length + (0.01)}em`
    }
    
</script>

<div class="h-28 grid grid-rows-2 w-full">
    <div class="flex justify-between bg-primary rounded-md mt-2 p-1 h-8">
        <svg
            class=""
            viewBox="0 0 {maxLength} 1">
            
            {#each currentClueState as char, index}
                <g transform="{`translate(${index}, 0)`}">
                    <rect 
                        stroke-width=".02"
                        text-anchor="center"
                        width={.9} 
                        height={.9}
                        rx=".1px"
                        ry=".1px"/>
                    
                    <text 
                        class="char"
                        font-size={getFontSize(char)}
                        x="0.45"
                        y="0.75"
                        text-anchor="middle"
                        font-weight="bold" >
                        {char}
                    </text>
                </g>
            {/each}
        </svg>
        <button onclick={() => gameManager.assistManager.getHint()} class="bg-white rounded-md px-2 hover:bg-selected">
            <span class="inline-block w-full h-full hover:-translate-y-0.5 transition-transform text-black">
                Hint
            </span>
        </button>
    </div>
    <div>
        <span class="font-bold">
            {gameManager.state.clue?.id}
        </span>
        {gameManager.state.clue?.hint}
    </div>
    
</div>

<style>
    rect {
        fill: var(--color-board);
        stroke: var(--color-block)
    }

    text {
        fill: var(--color-board-chars)
    }

</style>