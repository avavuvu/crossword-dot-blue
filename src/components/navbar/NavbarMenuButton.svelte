<script lang="ts">
    import type { Snippet } from "svelte";


    const { children, props, onclick, }: {
        children: Snippet,
        props?: any
        onclick: () => void
    } = $props()

</script>

<button
    {...props}
    {onclick}
    class="
        active:bg-white bg-white text-black hover:bg-selected
        midnight:hover:text-white
        cursor-pointer p-4 min-w-48 hover:translate-x-0.5 transition-transform">
    {@render children()}
</button>