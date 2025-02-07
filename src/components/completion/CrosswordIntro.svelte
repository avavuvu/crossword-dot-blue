<script lang="ts">
    import type { Crossword } from "$lib/types";
    import type { Snippet } from "svelte";
    import Chillies from "./Chillies.svelte";
    import { options } from "$lib/options.svelte";
    import { wordLogo } from "@/svg/wordLogo.svelte";


    const { children, crossword }: {children: Snippet, crossword: Crossword} = $props()
    const {metadata} = crossword

    const title = metadata.title ?? "Crossword"

</script>

<div class="max-h-12 mx-auto">
    {@render wordLogo()}
</div>

<h1 class="text-6xl">{title}</h1>

<span class="text-xl">
    <Chillies amount={metadata.difficulty}/>
</span>

{#if options.RebusWarning.setting}
    {#if crossword.rebus}
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


{@render children()}

<hr>

<p class="text-xl">
    <span class="font-bold">{metadata.date.toDateString()}</span> — by {metadata.author}
</p>