import type { ThemeSelect } from "@setups";

export const themeSelect: ThemeSelect = (form, { select }) => {
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
};
