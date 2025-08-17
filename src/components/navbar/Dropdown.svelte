<script lang="ts">
    import { Popover } from "melt/builders";
    import NavbarButton from "./NavbarButton.svelte";
    import NavbarMenuButton from "./NavbarMenuButton.svelte";

    const popover = new Popover()

    interface Props {
        label: string
        menuItems: {
            onclick: () => void,
            label: string
        }[]
	}

    const { label, menuItems }: Props = $props()
</script>


<NavbarButton props={popover.trigger}>
    <h1 >{label}</h1>
</NavbarButton>

<div class="shadow rounded-xl bg-white border-primary border-2" {...popover.content} >
    <ul class="overflow-clip">
        {#each menuItems as menuItem}
            <li>
                <NavbarMenuButton onclick={() => {
                    popover.open = false
                    menuItem.onclick()
                }}>
                    {menuItem.label}
                </NavbarMenuButton>
            </li>
        {/each}

    </ul>
</div>


<style>
	[data-melt-popover-content] {
		position: absolute;
		pointer-events: none;
		opacity: 0;

		transform: scale(0.9);

		transition: 0.3s;
		transition-property: opacity, transform;
		transform-origin: var(--melt-popover-content-transform-origin, center);
	}

	[data-melt-popover-content][data-open] {
		pointer-events: auto;
		opacity: 1;

		transform: scale(1);
	}
</style>