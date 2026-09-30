import type { MarkdownHighlight } from "@setups";

const highlight = () => import("./highlight");

export const markdownHighlight: MarkdownHighlight = async (editor, { backdrop, signal }) => {
    const textarea = () => editor.querySelector<HTMLTextAreaElement>(".editor > textarea");

    document.addEventListener(
        "htmx:after:settle",
        async () => {
            const current = textarea();
            if (!current) return;
            const { paint } = await highlight();
            paint(current, backdrop);
        },
        { signal },
    );

    const current = textarea();
    if (!current) return;

    const { attach } = await highlight();
    if (!signal.aborted) attach(current, backdrop);
};
