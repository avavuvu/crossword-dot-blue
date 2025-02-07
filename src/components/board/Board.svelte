<script lang="ts">
    import type { Crossword } from "$lib/types"
    import { gameManager } from "$lib/gameManager.svelte";
    import { indexToCoords } from "$lib/Coord";

    const { crossword }: { crossword: Crossword } = $props()

    const cells = crossword.grid.map((cell, index) => {
        const [x,y] = indexToCoords(index, crossword.width)

        const onclick = () => {
            if(cell.solid) {
                return () => {}
            }

            return gameManager.selectCell(index)
        }

        return { ...cell, x, y, onclick }
    })
</script>

<g>
{#each cells as {gray, circled, solid, x, y, onclick}}
    <!-- svelte-ignore attribute_global_event_reference -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <g
        {x}
        {y}

        {onclick}

        class="square"
        role="cell"
        tabindex="0"
        class:solid={solid}
        transform="{`translate(${x}, ${y})`}"
        >
        <rect
            width={1} 
            height={1}>
        </rect>
    </g>

    
    
{/each}
</g>


<style>
    g {
        cursor: pointer;
        user-select: none;
        touch-action: manipulation;
    }

    rect {
        stroke-width: .01px;
        fill: var(--color-board);
        stroke: var(--color-block);
        transition: fill 0.05s ease-out;

    }

    .solid rect {
        fill: var(--color-block);
    }
    

    g:focus {
        outline: none;
    }
</style>