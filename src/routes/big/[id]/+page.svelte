<script>
    import CrosswordPage from "@/CrosswordPage.svelte";

    const { data } = $props()
    const { id } = data
</script>

<CrosswordPage collection="big" {id}/>