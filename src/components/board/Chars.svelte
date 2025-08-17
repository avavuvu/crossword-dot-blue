<script lang="ts">
    import type { Crossword } from "$lib/game/types";

    const { crossword }: { crossword: Crossword } = $props()

    import { gameManager } from "$lib/game/gameManager.svelte";
    import { indexToCoords } from "$lib/game/Coord";
    import { fade, fly, type FadeParams, scale, type ScaleParams, slide, type SlideParams } from "svelte/transition";
    import { cubicInOut, elasticOut } from "svelte/easing";

    const whoosh = (node: Element, params: { delay?: number, duration?: number, easing?: (t: number) => number }) => {
		const existingTransform = getComputedStyle(node).transform.replace('none', '');

		return {
			delay: params.delay || 0,
			duration: params.duration || 400,
			easing: params.easing || cubicInOut,
            // u is 1 - t
			css: (t: number, u: number) => `
                transform: ${existingTransform}
                    translateX(${-u*10}px)
                    scaleX(1);
                opacity: ${t*100}%;
                
            `
		};
	}

    const animationMultiplier = 50

    const cells = $derived(gameManager.state.grid.map((char, index) => {
        const [x, y] = indexToCoords(index, crossword.width)

        const checked = gameManager.state.checked.get(index)

        const fadeParams: ScaleParams = {
            "duration": (x * animationMultiplier) + (y * animationMultiplier),
            "start": 10
        }

        const transition = checked ? whoosh : () => ({})
        
        return {char, x, y, checked, transition, fadeParams}
    }))

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

<g>
{#each cells as {char, x, y, checked, transition, fadeParams}, index}
    <g
        {x}
        {y}
        role="cell"
        tabindex="0"
        transform="{`translate(${x}, ${y})`}"
        >


        {#if char}
            <text
                class:incorrect={checked === false}
                class:correct={checked === true}
                class="char"
                font-size={getFontSize(char)}

                in:transition={fadeParams}
                x="0.5"
                y="0.82"
                text-anchor="middle"
                font-weight="bold">

                {char.toUpperCase()}
            </text>
        {/if}
    </g>
    
{/each}
</g>

<style>
    text {
        pointer-events: none;
        user-select: none;
        line-height: 1;
        font-family: Helvetica, Arial, sans-serif;
    }

    text {
        fill: var(--color-board-chars)
    }

    text.correct {
        fill: var(--color-correct);
    }

    text.incorrect {
        fill: var(--color-incorrect)
    }

    .char {
        
        font-weight: 400;
        /* font-weight: bolder; */
    }
</style>