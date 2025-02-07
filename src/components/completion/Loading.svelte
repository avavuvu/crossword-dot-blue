<script lang="ts">
    const loadingText = [..."LOADING"]
</script>

<svg
    class=""
    viewBox="0 0 {loadingText.length} 1">
    
    {#each loadingText as char, index}
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
                font-size="0.05em"
                style="animation: loading 2s ease-in-out {index * 10}ms infinite"
                x="0.45"
                y="0.75"
                text-anchor="middle">
                {char}
            </text>
        </g>
    {/each}
</svg>

<style>
    rect {
        fill: var(--color-board);
        stroke: var(--color-block)
    }

    text {
        fill: var(--color-board-chars);
    }

    @keyframes -global-loading {
        50% {
            fill: var(--color-primary);
        }
    }
</style>