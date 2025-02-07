<script lang="ts">
    import { gameManager } from "$lib/gameManager.svelte";

    let elapsedTimeInSeconds = $state(0)
    let formattedTime = $state("00")

    $effect(() => {
        gameManager.state.elapsedTime

        const inSeconds = gameManager.timeManager.timeInSeconds
        if(inSeconds >= elapsedTimeInSeconds) {
            formattedTime = gameManager.timeManager.formattedTime
        }
    })

    $effect(() => {
        if(gameManager.state.completion === "won") {
            gameManager.timeManager.pause()
        }
    })
</script>

<svg
    class="max-h-8 inline"
    viewBox="0 0 5 1">
    
    {#each [...formattedTime] as char, index}
        <g transform="{`translate(${index}, 0)`}">
            <rect 
                stroke-width=".02"
                text-anchor="center"
                width={.9} 
                height={.9}
                rx=".1px"
                ry=".1px"/>
            
            <text 
                class="char fill-black midnight:fill-white"
                font-size=".05em"
                x="0.45"
                y="0.75"
                text-anchor="middle"
                font-weight="bold" >
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
</style>