<script lang="ts">
    import CrosswordContainer from "@/CrosswordContainer.svelte";
    import { CrosswordAPI } from "$lib/crosswordLoader";
    import Navbar from "./navbar/Navbar.svelte";
    import { options } from "$lib/options.svelte";
    import Footer from "./Footer.svelte";
    import { Collapsible } from "melt/builders";
    import NavbarButton from "./navbar/NavbarButton.svelte";
    import { Settings } from "lucide-svelte";
    import Loading from "./completion/Loading.svelte";
    import type { CrosswordCollection } from "$lib/types";

    const { collection, id }: {
        collection: CrosswordCollection
        id: string
    } = $props()

    const collapsible = new Collapsible()
</script>


<div>
    <Navbar archive={collection}>
        {#snippet optionsSnippet()}
            <NavbarButton props={collapsible.trigger}>
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
    <CrosswordContainer {crossword} {collapsible}/>
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


