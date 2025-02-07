<script lang="ts">
    const { amount }: {amount: number}  = $props()

    const emptyChillies = amount
</script>

<span class='inline-flex justify-center items-center max-h-8 w-min mx-auto'>
    <span class="mr-2">
        Difficulty:

    </span>
    {#each Array.from({length: 5}, (_, index) => index < emptyChillies) as isEmpty}
        <img class="min-w-8" src={
            isEmpty 
                ? "/icons/chilli-fill.svg"
                : "/icons/chilli-empty.svg"
        }
        alt={
            isEmpty 
                ? "Filled chilli icon"
                : "Unfilled chilli icon"
        }>
    {/each}
</span>