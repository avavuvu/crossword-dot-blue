<script lang="ts">
    import type { Cell } from "$lib/game/types";

    const { width, height, grid}: {
        width: number,
        height: number,
        grid: Cell[]
    } = $props()

    const generateSquares = () => {
        return grid.map((cell, index) => ({
            x: index % width,
            y: Math.floor(index / width),
            cell
        }))
    }
</script>

    <svg viewBox="0 0 {width} {height}">
        {#each generateSquares() as {x, y, cell}}
        <g
            {x}
            {y}
            
            class="square"
            class:solid={cell.solid}
            transform="{`translate(${x}, ${y})`}"
            >
            <rect 
                width={1} 
                height={1}>
            </rect>
        </g>
    
        {/each}
    </svg>

<style>
    g {
        user-select: none;
    }

    circle {
        fill: none;
        stroke: var(--color-block);
        stroke-width: .1px;
    }

    rect {
        stroke-width: .1px;
        stroke: var(--color-block);
        fill: var(--color-board)
    }

    .solid rect {
        fill: var(--color-block);
    }

    @keyframes -global-text-loading {
        50% {
            opacity: 100%;
        }
    }
</style>