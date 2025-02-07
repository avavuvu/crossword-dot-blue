<script>

    import Navbar from "@/navbar/Navbar.svelte";
    import NavbarButton from "@/navbar/NavbarButton.svelte";
    import Options from "@/options/Options.svelte";
    import { Settings } from "lucide-svelte";

</script>
<Navbar>
    {#snippet optionsSnippet()}
        <NavbarButton props={{}}>
            <a href="/" class="w-full h-full flex justify-center items-center">
                <h1><Settings/></h1>
            </a>
        </NavbarButton>
    {/snippet}
</Navbar>


<Options/>
