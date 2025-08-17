<script lang="ts">
    import { indexToCoords } from "$lib/game/Coord";
    import { gameManager } from "$lib/game/gameManager.svelte";
    import type { Clue, Crossword } from "$lib/game/types";

    const { crossword } : {crossword: Crossword} = $props()

    const [x, y] = $derived.by(() => {
        const currentIndex = gameManager.state.clue?.indexes[gameManager.state.indexInClue] 
        if(!currentIndex) { return [-1,-1] }

        const x = currentIndex % crossword.width
        const y = Math.floor(currentIndex / crossword.width)

        return [x, y]
    })

    // i fucking love how awfully illegible this code is :>
    const selectedCells = $derived(
        gameManager.state.clue?.indexes
            .map(index => indexToCoords(index, crossword.width)) ?? [])

    const linkedCells = $derived(
            (crossword.links.find(links => links
                .some(({id}) => gameManager.state.clue?.id === id)) 
            ?? [])
        .flatMap(clue => clue.indexes.map((index) => indexToCoords(index, crossword.width)))
    )

</script>

<g class="cursor">
    {#each linkedCells as [x, y]}
    <g
        {x}
        {y}
        
        class="linked cursor"
        role="cell"
        tabindex="0"
        transform="{`translate(${x}, ${y})`}"
        >
        <rect
            width={1} 
            height={1}>
        </rect>
    </g>
    {/each}

    {#each selectedCells as [x, y], index}
        <g
            {x}
            {y}
            
            class="secondary cursor"
            class:pointer={index === gameManager.state.indexInClue}
            role="cell"
            tabindex="0"
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
    .cursor {
        pointer-events: none;
        user-select: none;
    }

    .linked {
        fill: var(--color-linked);
        stroke: var(--color-primary);
        stroke-width: 0.01;
    }

    .secondary {
        fill: var(--color-secondary-selected);
        stroke-width: .04;
        stroke: var(--color-selected);
    }

    .pointer {
        fill: var(--color-selected);
    }

</style>