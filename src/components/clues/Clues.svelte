<script lang="ts">

    import { gameManager } from "$lib/game/gameManager.svelte";
    import type { Clue, Crossword } from "$lib/game/types";

    const { crossword }: { crossword: Crossword } = $props()

    const downClues = crossword.clues.filter(({isHorizontal}) => !isHorizontal)
    const acrossClues = crossword.clues.filter(({isHorizontal}) => isHorizontal)

    type SelectedOption = "selected" | "opposite" | "none"
    const isClueSelected = (clue: Clue): SelectedOption => {
        if(!gameManager.state.clue) {
            return "none"
        }

        if(gameManager.state.clue?.id === clue.id) {
            return "selected"
        }

        if(clue.indexes.includes(gameManager.state.cursorIndex!)) {
            return "opposite"
        }

        return "none"
    }

    const isLinked = (clue: Clue) => {
        if(!crossword.links) { return false }
        if(!gameManager.state.clue) { return false }

        const relevantLinkGroup = crossword.links
            .find(links => links.some(({id}) => id === gameManager.state.clue?.id))

        if(!relevantLinkGroup) { return false }

        return relevantLinkGroup.includes(clue)
    }

    $effect(() => {
        const id = gameManager.state.clue?.id
        
        if(!id) {
            return
        }

        if(id === "") {
            return
        }

        const clueElement = document.getElementById(id)
        if(!clueElement) {
            return
        }

        const columnElement = document.getElementById(`${gameManager.state.clue!.isHorizontal ? "across" : "down"}-column`)

        columnElement!.scrollTo({
            "top": clueElement.offsetTop - columnElement!.offsetTop,
            "behavior": "smooth"
        })

    })

</script>

{#snippet clueColumn(clues: Clue[], title: string)}
    <div id="{title}-column" class="min-w-[15ch] overflow-y-scroll min-h-0">
        <h2 class="text-3xl font-bold capitalize sticky top-0 bg-white">{title}</h2>
        <ul class="text-left ">
            {#each clues as clue}
                <li 
                    id={clue.id}
                    class:selected={isClueSelected(clue) === "selected"}
                    class:opposite={isClueSelected(clue) === "opposite"}
                    class:linked={isLinked(clue)}
                    class="rounded-md">
                    <button
                        class={`flex w-full cursor-pointer 
                            ${isClueSelected(clue) === "selected" ? "grove:text-white midnight:text-white" : ""}`}

                        onclick={() => { gameManager.clueManager.setClue(clue, { indexToNextEmpty: true } )}}>
    
                        <span class="font-bold w-[30px] min-w-[30px] text-right">
                            {clue.id.slice(0,-1)}.
                        </span> 
    
                        <span class="w-2"></span>
    
                        <span class="p-0 text-left">
                            {clue.hint}
                        </span>
                    </button>
                </li>
            {/each}
        </ul>
    
    </div>
{/snippet}

{@render clueColumn(acrossClues, "across")}
{@render clueColumn(downClues, "down")}


<style>
    .linked {
        background-color: var(--color-linked);
    }
    .opposite {
        background: repeating-linear-gradient(
            45deg,
            rgba(0, 0, 0, 0.1),
            rgba(0, 0, 0, 0.1) 10px,
            rgba(0, 0, 0, 0.15) 10px,
            rgba(0, 0, 0, 0.15) 20px
            )
    }
    .selected {
        background-color: var(--color-selected);
    }
    
</style>
