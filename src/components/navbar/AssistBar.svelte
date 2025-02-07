<script lang="ts">
    import { gameManager } from "$lib/gameManager.svelte";
    import Dropdown from "./Dropdown.svelte";

    const { rebus }: { rebus: boolean} = $props()

    const checkItems = [
            {
                label: "Check All",
                onclick: () => gameManager.assistManager.checkPuzzle()
            },
            {
                label: "Check Word",
                onclick: () => gameManager.assistManager.checkWord()
            },
            {
                label: "Check Square",
                onclick: () => gameManager.assistManager.checkSelected()
            }
        ]
    
    const revealItems = [
            {
                label: "Reveal All",
                onclick: () => gameManager.assistManager.revealPuzzle()
            },
            {
                label: "Reveal Word",
                onclick: () => gameManager.assistManager.revealWord()
            },
            {
                label: "Reveal Square",
                onclick: () => gameManager.assistManager.revealSelected()
            }
        ]

    const resetItems = [
            {
                label: "Reset Puzzle",
                onclick: () => gameManager.assistManager.resetPuzzle()
            }
        ]
</script>

{#if rebus}
<button class="" onclick={() => gameManager.state.isRebusMode = !gameManager.state.isRebusMode}>
    <div class={["transition-transform h-full flex items-center justify-center rounded-xl min-w-8 px-4",
                "hover:bg-white hover:text-black hover:-translate-y-0.5",
                "bg-selected",
                gameManager.state.isRebusMode ? "bg-white text-black -translate-y-0.5" : ""]}>
        Rebus
    </div>
</button>
{/if}


<Dropdown label={"Check"} menuItems={checkItems}/>
<Dropdown label={"Reveal"} menuItems={revealItems}/>
<Dropdown label={"Reset"} menuItems={resetItems}/>

