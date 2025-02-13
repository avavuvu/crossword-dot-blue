<script lang="ts">
    import { gameManager } from "$lib/gameManager.svelte";
    import { inputKey } from "$lib/input";
    import type { CharacterSet, Crossword } from "$lib/types";
    import { Delete, Globe, IterationCw } from "lucide-svelte";
    import type { Icon as IconType } from "lucide-svelte";

    const { crossword }: { crossword: Crossword } = $props()

    const rebusRow = "Rebus Toggle"

    let defaultLayout: string =
 `Q W E R T Y U I O P
A S D F G H J K L
Symbols Z X C V B N M Backspace`
    

    let symbolsLayout: string = 
`1 2 3 4 5 6 7 8 9 0
 - / : ; ( ) $ & @
Symbols . , ? ! ' \" # Backspace`
    

    const replaceSymbols = (set: string) => {
        return set.replace(/Symbols/g, "Toggle")
    }

    if(crossword.rebus) {
        symbolsLayout += `\n${rebusRow}`
        defaultLayout += `\n${rebusRow}`
    }

    if(crossword.characterSet === "default") {
        symbolsLayout = replaceSymbols(symbolsLayout)
        defaultLayout = replaceSymbols(defaultLayout)
    }

    const icons: {[key: string]: typeof IconType} = {
        "Backspace": Delete,
        "Symbols": Globe,
        "Toggle": IterationCw
    }

    

    type Layout = {
        display: string | typeof IconType, 
        noThroughputKey: boolean,
        useIcon: boolean
        nonCharKey: boolean
        value: string,
    }[][]

    const createLayoutArray = (layoutString: string): Layout => layoutString.split("\n").map(row => (
            row.split(" ").map(keyChar => {
                const useIcon = icons[keyChar] !== undefined

                return {
                    useIcon,
                    display: useIcon ? icons[keyChar] : keyChar,
                    value: keyChar,
                    nonCharKey: useIcon || keyChar.length > 1,
                    noThroughputKey: keyChar === "Symbols",
                }
            })
        ))

    
    
    const defaultlayoutArray: Layout = createLayoutArray(defaultLayout)
    const symbolsLayoutArray: Layout = createLayoutArray(symbolsLayout)
    let enableSymbolLayout = $state(false)
    let layoutArray = $derived(enableSymbolLayout ? symbolsLayoutArray : defaultlayoutArray)

</script>

<div class="w-screen z-50
    bg-linked 
    dark:bg-white 
    midnight:bg-white">
    <div class="px-[12px] py-[16px]">
        {#each layoutArray as row}
            <div class="flex mb-[12px]">
                {#each row as {value, display, nonCharKey, useIcon, noThroughputKey}}
                {@const Icon = display as typeof IconType}
                    <button 
                        onclick={() => {
                            if(noThroughputKey) {
                                if(value === "Symbols") {
                                    enableSymbolLayout = !enableSymbolLayout
                                }

                                return
                            }

                            inputKey(new KeyboardEvent("keydown", { key: value }))
                        }}
                        class:special-key={nonCharKey}
                        class:rebus-key={value === "Rebus" && gameManager.state.isRebusMode}
                        class="
                        not-last:mr-[4px] pt-[8px]
                        flex justify-center w-[20px] 
                        min-h-[40px] min-w-[20px] grow  rounded-md shadow
                        bg-white 
                        dark:bg-slate-600 dark:text-black
                        midnight:bg-primary midnight:text-white 
                        active:scale-y-200 will-change-transform origin-bottom group">
                        <span class="pointer-events-none group-active:scale-y-50 group-active:-translate-y-3">
                            {#if useIcon}
                                <Icon strokeWidth="1"/>
                            {:else}
                                {display}
                            {/if}
                        </span>
                    </button>
                {/each}
            </div>
        {/each}

    </div>
</div>

<style>
    button {
        touch-action: manipulation;
    }

    .special-key {
        background-color: var(--color-selected);
    }

    .rebus-key {
        background-color: var(--color-white);
    }
</style>