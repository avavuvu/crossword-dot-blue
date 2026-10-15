import type { CopyButton, CopyLink } from "@setups";

function flash(button: HTMLElement, text: string): void {
    const original = button.textContent;
    button.textContent = text;
    setTimeout(() => (button.textContent = original), 1500);
}

export const copyButton: CopyButton = (button) => ({
    copy() {
        const url = button.dataset.copy!;
        if (button.hasAttribute("data-share") && navigator.share) {
            navigator.share({ url }).catch(() => {});
        } else {
            navigator.clipboard
                .writeText(url)
                .then(() => flash(button, button.dataset.copied ?? "Copied"))
                .catch(() => {});
        }
    },
});

export const copyLink: CopyLink = () => ({
    select(_event, input) {
        input.select();
    },
});
