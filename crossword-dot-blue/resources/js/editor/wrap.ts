export type Edit = {
    text: string;
    start: number;
    end: number;
    replaceStart: number;
    replaceEnd: number;
};

export function toggleWrap(value: string, start: number, end: number, marker: string): Edit {
    const selected = value.slice(start, end);
    const n = marker.length;

    const before = value.slice(start - n, start);
    const after = value.slice(end, end + n);
    if (before === marker && after === marker) {
        return { text: selected, start: start - n, end: end - n, replaceStart: start - n, replaceEnd: end + n };
    }

    if (selected.startsWith(marker) && selected.endsWith(marker) && selected.length >= 2 * n) {
        const inner = selected.slice(n, -n);
        return { text: inner, start, end: start + inner.length, replaceStart: start, replaceEnd: end };
    }

    return {
        text: marker + selected + marker,
        start: start + n,
        end: end + n,
        replaceStart: start,
        replaceEnd: end,
    };
}

export function wrapLink(value: string, start: number, end: number): Edit {
    const selected = value.slice(start, end);
    const looksLikeUrl = /^(https?:\/\/|mailto:|\/)\S+$/i.test(selected);

    if (looksLikeUrl) {
        const text = `[](${selected})`;
        return { text, start: start + 1, end: start + 1, replaceStart: start, replaceEnd: end };
    }

    const text = `[${selected}](url)`;
    const urlStart = start + selected.length + 3;
    return { text, start: urlStart, end: urlStart + 3, replaceStart: start, replaceEnd: end };
}

export function applyEdit(textarea: HTMLTextAreaElement, edit: Edit): void {
    textarea.setRangeText(edit.text, edit.replaceStart, edit.replaceEnd, "preserve");
    textarea.setSelectionRange(edit.start, edit.end);
    textarea.dispatchEvent(new InputEvent("input", { bubbles: true, inputType: "insertText" }));
}
