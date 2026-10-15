import type { DetailsLink } from "@setups";

export const detailsLink: DetailsLink = (link) => ({
    open() {
        const details = document.getElementById(link.dataset.open!);
        if (details instanceof HTMLDetailsElement) details.open = true;
    },
});
