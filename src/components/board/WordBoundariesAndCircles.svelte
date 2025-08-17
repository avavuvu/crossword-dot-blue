<script lang="ts">
    import { indexToCoords } from "$lib/game/Coord";
    import { options } from "$lib/options.svelte";
    import type { Crossword } from "$lib/game/types";

    const { crossword }: { crossword: Crossword } = $props()

    const wordBoundaries = crossword.grid.flatMap((cell, index) => {
        const [x, y] = indexToCoords(index, crossword.width)

        const { across, down } = cell.wordBoundary

        return {across, down, x, y, circled: cell.circled}
    })
</script>

<g>
    {#each wordBoundaries as {x, y, across, down, circled}}
        <g
            {x}
            {y}
            role="cell"
            tabindex="0"
            transform="{`translate(${x}, ${y})`}"
            >
    
            {#if options.WordBoundaries.setting}
                {#if across}
                    <line 
                        x1={1}
                        y1={.1}
                        x2={1}
                        y2={.9}

                        stroke="black" 
                        stroke-dasharray=".08"
                        stroke-dashoffset=".1"
                        stroke-width="0.004rem"/>
                {/if}

                {#if down}
                    <line 
                        x1={.1}
                        y1={1}
                        x2={.9}
                        y2={1}

                        stroke="black" 
                        stroke-dasharray=".08"
                        stroke-dashoffset=".1"
                        stroke-width="0.004rem"/>
                {/if}
            {/if}

            {#if circled}
                <circle cx=".5" cy=".5" r=".45"/>
            {/if}
        </g>
        
    {/each}
</g>

<style>
    circle {
        fill: none;
        stroke: var(--color-block);
        stroke-width: .01px;
    }
</style>