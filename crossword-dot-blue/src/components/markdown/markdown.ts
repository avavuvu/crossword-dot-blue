const highlight = () => import("./_highlight");

export class CwMarkdown extends HTMLElement {
    async connectedCallback(): Promise<void> {
        const [textarea, backdrop] = this.parts();
        if (!textarea || !backdrop) return;

        const { attach } = await highlight();
        if (this.isConnected) attach(textarea, backdrop);
    }

    async paint(): Promise<void> {
        const [textarea, backdrop] = this.parts();
        if (!textarea || !backdrop) return;

        const { paint } = await highlight();
        paint(textarea, backdrop);
    }

    private parts(): [HTMLTextAreaElement | null, HTMLElement | null] {
        return [
            this.querySelector<HTMLTextAreaElement>(".editor > textarea"),
            this.querySelector<HTMLElement>(".editor > .backdrop"),
        ];
    }
}

if (!customElements.get("cw-markdown")) {
    customElements.define("cw-markdown", CwMarkdown);
}

document.addEventListener("htmx:after:settle", () => {
    for (const editor of document.querySelectorAll<CwMarkdown>("cw-markdown")) editor.paint();
});
