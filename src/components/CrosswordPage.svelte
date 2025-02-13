<script lang="ts">
    import { CrosswordAPI } from "$lib/crosswordLoader";
    import Navbar from "./navbar/Navbar.svelte";
    import { options } from "$lib/options.svelte";
    import NavbarButton from "./navbar/NavbarButton.svelte";
    import { Settings } from "lucide-svelte";
    import Loading from "./completion/Loading.svelte";
    import type { CrosswordCollection } from "$lib/types";
    import LandscapeCrosswordContainer from "./LandscapeCrosswordContainer.svelte";
    import PortraitCrosswordContainer from "./PortraitCrosswordContainer.svelte";
    import { MediaQuery } from "svelte/reactivity";

    const { collection, id }: {
        collection: CrosswordCollection
        id: string
    } = $props()

    let showSettings = $state(false)

    const isLargeScreen = new MediaQuery('min-width: 900px');
    const useMobile = $derived(!isLargeScreen.current || options.ForceMobile.setting)
</script>


<div>
    <Navbar archive={collection}>
        {#snippet optionsSnippet()}
            <NavbarButton props={{onclick: () => showSettings = !showSettings}}>
                <h1><Settings/></h1>
            </NavbarButton>
        {/snippet}
    </Navbar>
</div>

{#await CrosswordAPI.loadCrossword(collection, id)}
    <div class="min-h-screen max-w-96 p-8 mx-auto mt-24">
        <Loading/>
    </div>
{:then crossword} 
    {#if useMobile}
        <PortraitCrosswordContainer {crossword} bind:showSettings/>
    {:else}
        <LandscapeCrosswordContainer {crossword} bind:showSettings/>
    {/if}

    <svetle:head>
        <title>
            {crossword.metadata.date.toLocaleDateString('default', { 
                day: "2-digit", 
                month: "long", 
                weekday: "long"
            })} – Crossword Dot Blue
        </title>
    </svetle:head>
    
{:catch e}
    <div class="min-h-screen max-w-96 p-8 mx-auto mt-24">
        <h1 class="text-incorrect text-4xl">ERROR</h1>
        <p>An error occured when loading the puzzle. Please contact the webmaster.</p>
        <p>
            <span class="text-primary/50">
                {(e as Error).message}
            </span>
        </p>
    </div>
{/await}


