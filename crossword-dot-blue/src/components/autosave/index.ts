import type { AutosaveForm, LeaveLink } from "@setups";

const FORM = "form[data-autosave]";
const DIRTY = "data-dirty";
const PENDING = "htmx-request";
const UNSAVED = `${FORM}[${DIRTY}], ${FORM}.${PENDING}`;

function unsaved(): HTMLFormElement[] {
    return Array.from(document.querySelectorAll<HTMLFormElement>(UNSAVED));
}

function flushAll(): Promise<void> {
    const forms = unsaved();
    if (forms.length === 0) return Promise.resolve();

    return new Promise((resolve) => {
        const done = () => {
            document.removeEventListener("htmx:finally:request", check);
            clearTimeout(deadline);
            resolve();
        };
        const check = () => {
            if (unsaved().length === 0) done();
        };
        document.addEventListener("htmx:finally:request", check);
        const deadline = setTimeout(done, 3000);

        for (const form of forms) {
            if (form.hasAttribute(DIRTY) && !form.classList.contains(PENDING)) form.requestSubmit();
        }
    });
}

export const autosaveForm: AutosaveForm = (form) => ({
    input() {
        form.setAttribute(DIRTY, "");
    },
    sent() {
        form.removeAttribute(DIRTY);
    },
    failed() {
        form.setAttribute(DIRTY, "");
    },
    submit(event) {
        if (event.shiftKey) return;
        event.preventDefault();
        form.requestSubmit();
    },
    unload(event) {
        if (form.matches(UNSAVED)) event.preventDefault();
    },
});

export const leaveLink: LeaveLink = (link) => ({
    async leave(event) {
        if (unsaved().length === 0) return;

        event.preventDefault();
        link.setAttribute("aria-busy", "true");
        await flushAll();
        window.location.href = link.getAttribute("href") ?? window.location.href;
    },
});
