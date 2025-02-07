<script lang="ts">
    import Crossword from "@/board/Crossword.svelte";
    import Input from "@/input/Input.svelte";
    import Navbar from "@/navbar/Navbar.svelte";
    import NavbarButton from "@/navbar/NavbarButton.svelte";
    import Options from "@/options/Options.svelte";
    import { Collapsible } from "melt/builders";
    import { fly } from "svelte/transition";
    import { gameManager } from "$lib/gameManager.svelte";
    import CrosswordIntro from "@/completion/CrosswordIntro.svelte";
    import { options } from "$lib/options.svelte";
    import { Settings } from "lucide-svelte";
    import type { Crossword as CrosswordData } from "$lib/types";

    const { crossword, collapsible }: { 
        crossword: CrosswordData 
        collapsible: Collapsible
    } = $props()
    gameManager.init(crossword)

    

    let showIntro = $state(gameManager.state.completion !== "won")

    const startCrossword = () => { 
        showIntro = false; 
        collapsible.open = false
        gameManager.timeManager.start()
    }

</script>



<div {...collapsible.root} class=" text-black">
    {#if collapsible.open}
        <div class="w-full col-start-1 col-end-2 row-start-2 row-end-3 min-h-screen" {...collapsible.content} 
            in:fly={{ x: 100, duration: 200, delay: 150 }}
            out:fly={{ x: 100, duration: 200}}>
            
            <Options/>
        </div>
    {:else}
        <div class="grid col-start-1 col-end-2 row-start-2 row-end-3 min-h-screen" 
            in:fly={{ x: 100, duration: 200, delay: 150 }}
            out:fly={{ x: 100, duration: 200}}>

            {#if showIntro}
                <div 
                    class="col-start-1 col-end-2 row-start-1 row-end-2" 
                    in:fly={{ x: 100, duration: 200, delay: 150 }}
                    out:fly={{ x: 100, duration: 200}}>

                    <div class="mx-auto max-w-main flex justify-center flex-col mt-20 gap-4 text-center">
                        <CrosswordIntro {crossword}>
                            <button 
                                onclick={startCrossword}
                                class="block transition-all mx-auto w-24">
                                <div class="hover:-translate-y-0.5 bg-primary transition-transform h-full flex items-center justify-center 
                                    hover:bg-white hover:text-black text-white rounded-xl min-w-8 p-4">
                                    Play
                                </div>
                            </button>
                        </CrosswordIntro>


                    </div>
                    
                </div>
            {:else}
                <div class="grid col-start-1 col-end-2 row-start-1 row-end-2 min-h-screen" >
                    <Crossword {crossword}/>
                    <Input {crossword}/>
                </div>
            {/if}
        </div>
    {/if}
</div>

