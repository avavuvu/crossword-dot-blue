// inline markdown for clue text: **strong**, *em*, [text](url). emphasis follows
// the commonmark delimiter algorithm for `*` runs so the output matches the server.

export type RenderOptions = {
    keepMarkers?: boolean;
};

const LINK = /\[([^\[\]]*)\]\(([^\s()]*(?:\([^\s()]*\)[^\s()]*)*)\)/g;

export function escapeHtml(text: string): string {
    return text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");
}

export function safeHref(href: string): boolean {
    const lower = href.trim().toLowerCase();
    return (
        lower.startsWith("https://") ||
        lower.startsWith("http://") ||
        lower.startsWith("mailto:") ||
        (lower.startsWith("/") && !lower.startsWith("//"))
    );
}

export function renderInline(md: string, options: RenderOptions = {}): string {
    let out = "";
    let last = 0;

    LINK.lastIndex = 0;
    let match: RegExpExecArray | null;

    while ((match = LINK.exec(md))) {
        out += emphasis(md.slice(last, match.index), options);
        last = LINK.lastIndex;

        const [, text, href] = match as unknown as [string, string, string];
        const inner = emphasis(text, options);
        const mark = (s: string) => (options.keepMarkers ? `<span class="marker">${escapeHtml(s)}</span>` : "");

        if (safeHref(href)) {
            out += `<a href="${escapeHtml(href)}">${mark("[")}${inner}${mark("]")}</a>${mark(`(${href})`)}`;
        } else {
            out += `${mark("[")}${inner}${mark("]")}${mark(`(${href})`)}`;
        }
    }

    return out + emphasis(md.slice(last), options);
}

type Delimiter = {
    index: number;
    count: number;
    canOpen: boolean;
    canClose: boolean;
    active: boolean;
};

type Piece = { text: string } | Delimiter;

const PUNCTUATION = /[!-/:-@[-`{-~\p{P}]/u;
const WHITESPACE = /\s/;

function classify(char: string | undefined): "space" | "punct" | "other" {
    if (char === undefined || WHITESPACE.test(char)) return "space";
    if (PUNCTUATION.test(char)) return "punct";
    return "other";
}

function emphasis(text: string, { keepMarkers = false }: RenderOptions): string {
    const pieces: Piece[] = [];
    const delimiters: Delimiter[] = [];
    let i = 0;

    while (i < text.length) {
        if (text[i] !== "*") {
            let j = i;
            while (j < text.length && text[j] !== "*") j++;
            pieces.push({ text: text.slice(i, j) });
            i = j;
            continue;
        }

        let j = i;
        while (j < text.length && text[j] === "*") j++;

        const before = classify(text[i - 1]);
        const after = classify(text[j]);
        const leftFlanking = after !== "space" && (after !== "punct" || before !== "other");
        const rightFlanking = before !== "space" && (before !== "punct" || after !== "other");

        const delimiter: Delimiter = {
            index: pieces.length,
            count: j - i,
            canOpen: leftFlanking,
            canClose: rightFlanking,
            active: true,
        };
        pieces.push(delimiter);
        delimiters.push(delimiter);
        i = j;
    }

    const opens = new Map<number, string[]>();
    const closes = new Map<number, string[]>();

    const mark = (count: number) => (keepMarkers ? `<span class="marker">${"*".repeat(count)}</span>` : "");

    for (let c = 0; c < delimiters.length; c++) {
        const closer = delimiters[c]!;
        if (!closer.canClose) continue;

        let matched = false;
        for (let o = c - 1; o >= 0; o--) {
            const opener = delimiters[o]!;
            if (!opener.active || !opener.canOpen || opener.count === 0) continue;

            const oddMatch =
                (opener.canClose || closer.canOpen) &&
                (opener.count + closer.count) % 3 === 0 &&
                !(opener.count % 3 === 0 && closer.count % 3 === 0);
            if (oddMatch) continue;

            while (opener.count > 0 && closer.count > 0) {
                const strong = opener.count >= 2 && closer.count >= 2;
                const used = strong ? 2 : 1;
                const tag = strong ? "strong" : "em";

                opens.set(opener.index, [`<${tag}>${mark(used)}`, ...(opens.get(opener.index) ?? [])]);
                closes.set(closer.index, [...(closes.get(closer.index) ?? []), `${mark(used)}</${tag}>`]);

                opener.count -= used;
                closer.count -= used;
            }

            for (let k = o + 1; k < c; k++) delimiters[k]!.active = false;

            matched = true;
            if (closer.count === 0) break;
        }

        if (!matched && !closer.canOpen) closer.active = false;
    }

    let out = "";
    for (const piece of pieces) {
        if ("text" in piece) {
            out += escapeHtml(piece.text);
            continue;
        }

        const literal = "*".repeat(piece.count);
        const opening = opens.get(piece.index) ?? [];
        const closing = closes.get(piece.index) ?? [];

        out += closing.join("") + escapeHtml(literal) + opening.join("");
    }

    return out;
}
