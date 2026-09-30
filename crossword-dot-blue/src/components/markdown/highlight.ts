import { renderInline } from "../../../resources/js/editor/markdown";
import { applyEdit, toggleWrap, wrapLink } from "../../../resources/js/editor/wrap";

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

export function paint(textarea: HTMLTextAreaElement, backdrop: HTMLElement): void {
    backdrop.innerHTML = renderInline(textarea.value, { keepMarkers: true }) + "\n";
}

export function attach(textarea: HTMLTextAreaElement, backdrop: HTMLElement): void {
    if (textarea.classList.contains("highlighted")) {
        paint(textarea, backdrop);
        return;
    }

    textarea.classList.add("highlighted");
    textarea.addEventListener("input", () => paint(textarea, backdrop));
    textarea.addEventListener("scroll", () => {
        backdrop.scrollTop = textarea.scrollTop;
    });
    textarea.addEventListener("keydown", shortcuts);

    paint(textarea, backdrop);
}
