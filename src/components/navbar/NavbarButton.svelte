<script lang="ts">
    import type { Snippet } from "svelte";

    interface NavbarButtonProps {
        children: Snippet, 
        props: any,
    }

    const { children, props }: NavbarButtonProps = $props()

    
</script>

<button 
    {...props}
    class="block transition-all cursor-pointer">
    <div class="hover:-translate-y-0.5 transition-transform h-full flex items-center justify-center 
        hover:bg-white hover:text-black rounded-xl min-w-24 mx-0 lg:mx-4">
        {@render children()}
    </div>
</button>