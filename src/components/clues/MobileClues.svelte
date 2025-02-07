<script>

    import { gameManager } from "$lib/gameManager.svelte";
    import { ChevronRight, ChevronLeft } from "lucide-svelte";

</script>


<div class="w-screen bg-selected grid grid-cols-[15%_auto_15%] midnight:text-white ">
    <button 
        class="flex justify-center items-center
            active:bg-white"
        onclick={() => gameManager.clueManager.cycleClues(-1)}>
        <ChevronLeft strokeWidth="2"/>
    </button>
    <button 
        class="min-h-[60px]
            active:bg-white"
        onclick={() => gameManager.clueManager.toggleOrientation()}>
        <span class="tracking-tight leading-snug ">
            {gameManager.state.clue?.hint}
        </span>
    </button>
    <button 
        class="flex justify-center items-center
            active:bg-white"
        onclick={() => gameManager.clueManager.cycleClues(1)}>
        <ChevronRight strokeWidth="2"/>
    </button>


</div>

<style> 
    button {
        touch-action: manipulation;
    }
</style>