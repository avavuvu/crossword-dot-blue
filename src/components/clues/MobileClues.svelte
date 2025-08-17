<script>

    import { gameManager } from "$lib/game/gameManager.svelte";
    import { ChevronRight, ChevronLeft } from "lucide-svelte";

    const { size, lineHeight} = $derived.by(() => {
        const length = (gameManager.state.clue?.hint.length ?? 0)

        if(length > 45) {
            return {
                size: "13px",
                lineHeight: "1em"
            }
        }

        if(length > 20) {
            return {
                size: "16px",
                lineHeight: "1.1"
            }
        }


        return {
                size: "24px",
                lineHeight: "1.375"
            }
        
    }
        
    )

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
            block
            active:bg-white"
        style:font-size={size} style:line-height={lineHeight}
        onclick={() => gameManager.clueManager.toggleOrientation()}>
        <span >
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