<script lang="ts">
    import Navbar from "./navbar/Navbar.svelte";
    import { options } from "$lib/options.svelte";
    import NavbarButton from "./navbar/NavbarButton.svelte";
    import { Settings } from "lucide-svelte";
    import type { Crossword, CrosswordCollection } from "$lib/game/types";
    import LandscapeCrosswordContainer from "./LandscapeCrosswordContainer.svelte";
    import PortraitCrosswordContainer from "./PortraitCrosswordContainer.svelte";
    import { MediaQuery } from "svelte/reactivity";

    const { collection, crossword }: {
        collection: CrosswordCollection
        crossword: Crossword
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

{#if useMobile}
    <PortraitCrosswordContainer {crossword} bind:showSettings/>
{:else}
    <LandscapeCrosswordContainer {crossword} bind:showSettings/>
{/if}

