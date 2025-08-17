<script lang="ts">
    import { MediaQuery } from "svelte/reactivity";
    import type { Snippet } from "svelte";
    import NavbarButton from "./NavbarButton.svelte";
    import type { CrosswordCollection } from "$lib/game/types";
    import { logo } from "@/svg/logo.svelte";

    const { children, fullLogo = false, archive }: { 
        fullLogo?: boolean,
        archive?: CrosswordCollection
        children?: Snippet
    } = $props()

    const large = new MediaQuery('min-width: 768px');

    
</script>

<div class="flex bg-selected  mx-auto max-w-main w-full font-bold
            h-16 lg:h-24 lg:rounded-3xl">
    
    <div class="flex flex-row w-full h-full">
        <a href="/" class="h-full p-2">
            {@render logo()}
        </a>
        {#if archive}
            <NavbarButton props={{}}>
                <a href="/{archive}/archive" class="w-full h-full flex justify-center items-center">
                    <h1>More Puzzles</h1>
                </a>
            </NavbarButton>
        {/if}
    </div>

    {#if children}
        {@render children()}

    {/if}
</div>