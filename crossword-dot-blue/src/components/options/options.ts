export class CwOptions extends HTMLElement {
    connectedCallback(): void {
        const form = this.querySelector<HTMLFormElement>("form");
        const select = this.querySelector<HTMLSelectElement>('select[name="theme"]');
        if (!form || !select) return;

        select.addEventListener("change", () => {
            const theme = select.value;
            if (theme) document.documentElement.dataset.theme = theme;
            else delete document.documentElement.dataset.theme;

            const body = new URLSearchParams();
            for (const [name, value] of new FormData(form)) {
                if (typeof value === "string") body.append(name, value);
            }

            void fetch(form.action, {
                method: "POST",
                credentials: "same-origin",
                headers: { accept: "application/json" },
                body,
            });
        });
    }
}

if (!customElements.get("cw-options")) {
    customElements.define("cw-options", CwOptions);
}
