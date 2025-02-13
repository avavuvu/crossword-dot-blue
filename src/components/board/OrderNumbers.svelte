<script lang="ts">
    import { indexToCoords } from "$lib/Coord";
    import type { Crossword } from "$lib/types";

    const { crossword }: { crossword: Crossword } = $props()

    const orderNumbers = crossword.clues.map((clue) => {
        const orderNumber = clue.id.slice(0, -1)

        const [x, y] = indexToCoords(clue.indexes[0], crossword.width)

        return {orderNumber, x, y}
    })
</script>

<g>
    {#each orderNumbers as {x, y, orderNumber}}
        <g
            {x}
            {y}
            role="cell"
            tabindex="0"
            transform="{`translate(${x}, ${y})`}"
            >
    
            <text
                class="char fill-black midnight:fill-white dark:fill-white"
                x="0.08" y="0.2"
                text-anchor="start"
                font-weight="bold">

                {orderNumber}
            </text>
        </g>
        
    {/each}
</g>

<style>
    text {
        pointer-events: none;
        user-select: none;
        line-height: 1;
        font-family: Arial, Helvetica, sans-serif;
    }

    .char {
        font-size: 0.012em;
        font-weight: 400;
        font-weight: bolder;
    }
</style>