<script lang="ts">
    import { options, setOptionsFromStorage, type SelectSetting, type Setting, type SettingCategory } from "$lib/options.svelte";
    import { ChevronLeft, ChevronRight } from "lucide-svelte";
    import { onMount } from "svelte";

    const stepSetting = (setting: SelectSetting, change: -1 | 1) => {
        const currentIndex = setting.options.findIndex(({value}) => value === setting.setting)
        const newIndex = (currentIndex + change + setting.options.length) % setting.options.length

        setting.setting = setting.options[newIndex].value
    }

    type SortedOptions = Map<SettingCategory, [string, Setting][]>

    const sortedOptions: SortedOptions = $derived(
        Object.entries(options)
            .reduce((categoryMap, [name, setting]) => {
                if (!categoryMap.has(setting.category)) {
                    categoryMap.set(setting.category, []);
                }

                categoryMap.get(setting.category)?.push([name, setting])

                return categoryMap

            }, new Map<SettingCategory, [string, Setting][]>())
    )

    const settingChanged = () => {
        localStorage.setItem("options", JSON.stringify(options))
    }
</script>

{#snippet settingSnippet(name: string, setting: Setting)}
    <div class="my-2 mx-4 flex flex-col w-[250px] md:lg:w-[300px] lg:w-[400px] border-2 border-primary rounded-xl overflow-clip max-w-96">
        
        <span class="inline-block font-bold mx-4">{setting.name}</span>

        <div class="bg-primary text-white py-2 px-4">
            {#if setting.type === "bool"}
                <div class="grid grid-cols-[10%_auto]">
                    <input type="checkbox"
                        class="relative appearance-none shrink-0 w-4 h-4 border-2  rounded-sm mt-1 bg-white ring-2
                                focus:outline-none focus:ring-offset-0 focus:ring-1 focus:ring-blue-100
                                checked:bg-selected checked:border-0"
                        bind:checked={setting.setting}
                        onchange={settingChanged}>
                    <span class="ml-2">
                        {setting.setting ? "On" : "Off"}
                    </span>
                </div>

            {:else if setting.type ==="select"}
                <div class="grid grid-cols-[10%_auto_10%]">
                    <button 
                        class="hover:text-white"
                        onclick={() => {
                            stepSetting(setting, -1)
                            settingChanged()
                        }}>
                        <ChevronLeft/>
                    </button>
                    <select
                        class="mx-2"
                        bind:value={setting.setting}
                        onchange={settingChanged}>
                        {#each setting.options as {value, name}}
                            <option {value}>
                                {name}
                            </option>
                        {/each}
                    </select>
                    <button 
                        class="hover:text-white"
                        onclick={() => {
                            stepSetting(setting, 1)
                            settingChanged()
                            }}>
                        <ChevronRight/>
                    </button>
                </div>
            {/if}
        </div>
    </div>
{/snippet}

<div class="mx-auto max-w-main min-h-[600px] midnight:text-black dark:text-black">
    <h1 class="font-bold text-4xl py-8 mx-4 lg:mx-12">Options</h1>

    <div class="flex flex-wrap gap-y-8 justify-center">
        {#each [...sortedOptions].toSorted(([a],[b]) => a.localeCompare(b)) as [settingCategory, settings]}
            <div>
                <h1 class="text-2xl mx-4">{settingCategory}</h1>
        
                {#each settings as [name, setting]}
                    {@render settingSnippet(name, setting)}
                {/each}
            </div>
        {/each}

    </div>
</div>