<script lang="ts">
    import type { Crossword } from "$lib/game/types";
    import type { Snippet } from "svelte";
    import Chillies from "./Chillies.svelte";
    import { options } from "$lib/options.svelte";
    import { wordLogo } from "@/svg/wordLogo.svelte";
  import { logo } from "@/svg/logo.svelte";


    const { crossword, startCrossword }: { crossword: Crossword, startCrossword: () => void} = $props()
    const {metadata} = crossword

    const title = metadata.name ?? "Crossword"

</script>

<div class="flex justify-center flex-col mt-20 gap-4 text-center">
    <div class="max-h-12 mx-auto">
        {@render logo()}
    </div>

    <h1 class="text-6xl ">{title}</h1>


    {#if options.RebusWarning.setting}
        {#if crossword.metadata.isRebus}
            <div class="max-w-[45ch] mx-auto border-2 border-primary rounded-xl">
                <h1 class="w-full bg-primary text-white rounded-t-lg">WARNING</h1>
                <div class="p-4">
                    <p>This crossword uses <span class="font-bold">Rebus squares</span>, meaning some answers will require more than one letter.</p>
                    <p>Use the Rebus button (or the ` key) to input these answers.</p>
                    <br>
                    <p class="text-black/50">You can disable this warning in the options menu</p>
                </div>
            </div>
        {/if}
        
    {/if}


    <button 
        onclick={startCrossword}
        class="block transition-all mx-auto w-24">
        <div class="hover:-translate-y-0.5 bg-primary transition-transform h-full flex items-center justify-center 
            hover:bg-white hover:text-black text-white rounded-xl min-w-8 p-4">
            Play
        </div>
    </button>

    <p class="text-xl">
        <span class="font-bold">{metadata.date.toDateString()}</span> — by {metadata.author}
    </p>

    <span class="text-xl">
        <Chillies amount={metadata.difficulty}/>
    </span>
</div>