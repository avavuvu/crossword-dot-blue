import { renderInline } from "./markdown";
import { applyEdit, toggleWrap, wrapLink } from "./wrap";

const ATTACHED = "data-highlighted";

function backdropOf(textarea: HTMLTextAreaElement): HTMLElement | null {
    return textarea.parentElement?.querySelector<HTMLElement>(":scope > .backdrop") ?? null;
}

function paint(textarea: HTMLTextAreaElement): void {
    const backdrop = backdropOf(textarea);
    if (backdrop) backdrop.innerHTML = renderInline(textarea.value, { keepMarkers: true }) + "\n";
}

export function attachHighlight(textarea: HTMLTextAreaElement): void {
    if (textarea.hasAttribute(ATTACHED)) {
        paint(textarea);
        return;
    }
    if (!backdropOf(textarea)) return;
    textarea.setAttribute(ATTACHED, "");

    textarea.addEventListener("input", () => paint(textarea));
    textarea.addEventListener("scroll", () => {
        const backdrop = backdropOf(textarea);
        if (backdrop) backdrop.scrollTop = textarea.scrollTop;
    });
    textarea.addEventListener("keydown", shortcuts);

    paint(textarea);
}

function shortcuts(event: KeyboardEvent): void {
    if (!(event.metaKey || event.ctrlKey) || event.altKey || event.shiftKey) return;

    const textarea = event.currentTarget as HTMLTextAreaElement;
    const { value, selectionStart: start, selectionEnd: end } = textarea;

    const edit = (() => {
        switch (event.key.toLowerCase()) {
            case "b":
                return toggleWrap(value, start, end, "**");
            case "i":
                return toggleWrap(value, start, end, "*");
            case "k":
                return wrapLink(value, start, end);
            default:
                return null;
        }
    })();

    if (!edit) return;
    event.preventDefault();
    applyEdit(textarea, edit);
}

export function attachAll(root: ParentNode): void {
    for (const textarea of root.querySelectorAll<HTMLTextAreaElement>("textarea[data-markdown]")) {
        attachHighlight(textarea);
    }
}
