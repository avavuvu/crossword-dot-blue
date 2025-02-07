<script lang="ts">

    import { inputKey } from "$lib/input";
    import type { Crossword } from "$lib/types";


    const { crossword }: { crossword: Crossword} = $props()

    const onkeydown = (event: KeyboardEvent) => {
        event.preventDefault()

        if(event.key === "`" && !crossword.rebus) {
            return
        }
        
        inputKey(event)
    }
</script>

<svelte:window {onkeydown}/>