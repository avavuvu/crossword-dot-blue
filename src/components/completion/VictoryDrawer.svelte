<script lang="ts">
    import { gameManager } from "$lib/game/gameManager.svelte";
    import { share } from "$lib/share";
    import { X as XOutIcon } from "lucide-svelte";
    

    let { drawerOpen = $bindable() }: { drawerOpen: boolean } = $props()

    const shareTime = async () => await share(navigator, { 
        time: gameManager.timeManager.formattedTime,
        url: "TODO: IMPLEMENT URL"
        // url: page.url.toString()
    })

    let shareSuccessful: boolean | null = $state(null)
    
</script>

<div class="flex flex-col text-center border-2 border-primary rounded-4xl rounded-tl-none overflow-hidden">
    <button class="w-8 aspect-square" onclick={() => drawerOpen = false}>
        <div class="hover:-translate-y-0.5 transition-transform h-full flex items-center justify-center 
                bg-primary text-white 
                hover:bg-white hover:text-black ">
                <XOutIcon/>
            </div>
    </button>
    
    {#if gameManager.state.completion === "won"}
        <div>
            <h1 class="text-3xl font-bold">
                Congratulations!
            </h1>
            
            <hr class="border-t-primary border-t-2">
            
            <p>You solve this puzzle in {gameManager.timeManager.formattedTime} seconds!</p>
        </div>

        <div>
            <button class="h-full min-w-8" onclick={async () => shareSuccessful = await shareTime()}>
                <div class="hover:-translate-y-0.5 transition-transform h-full flex items-center justify-center 
                    bg-primary text-white px-4 py-2
                    hover:bg-white hover:text-black rounded-xl ">
                    {#if shareSuccessful === null}
                        Share
                    {:else if shareSuccessful === false}
                        Copied to Clipboard!
                    {:else if shareSuccessful === true}
                        Shared!
                    {/if}

                </div>
            </button>
            <button class="h-full  min-w-8" onclick={() => gameManager.assistManager.resetPuzzle()}>
                <div class="hover:-translate-y-0.5 transition-transform h-full flex items-center justify-center 
                    bg-primary text-white px-4 py-2
                    hover:bg-white hover:text-black rounded-xl ">
                    Restart the Puzzle
                </div>
            </button>
        </div>

        <img class="h-full w-full mx-auto" src="/images/me.webp" alt=""/>

    {:else}
        <div class="">
            <h1 class="text-3xl font-bold">
                Not quite!
            </h1>
            
            <hr class="border-t-primary border-t-2">
            
            <p>At least one square is incorrect!</p>
        </div>
        
        <div class="">
            <button class="h-full  min-w-8" onclick={() => {
                gameManager.assistManager.checkPuzzle()
                drawerOpen = false
            }}>
                <div class="hover:-translate-y-0.5 transition-transform h-full flex items-center justify-center 
                    bg-primary text-white px-4 py-2
                    hover:bg-white hover:text-black rounded-xl ">
                    Highlight Errors
        
                </div>
            </button>
            <button class="h-full  min-w-8" onclick={() => drawerOpen = false}>
                <div class="hover:-translate-y-0.5 transition-transform h-full flex items-center justify-center 
                    bg-primary text-white px-4 py-2
                    hover:bg-white hover:text-black rounded-xl ">
                    Continue
                </div>
            </button>
        </div>
    {/if}
   

</div>