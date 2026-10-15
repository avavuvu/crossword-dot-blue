import type { Setup } from "./boutique/setups";

export type AutosaveForm = Setup<"form", {}, { failed: { event: "htmx:response:error"; tag: "form" }; input: { event: "input"; tag: "form" }; sent: { event: "htmx:before:request"; tag: "form" }; submit: { event: "keydown"; tag: "textarea" }; unload: { event: "beforeunload"; tag: "form"; target: "window" } }>;
export type CardProgress = Setup<"cw-card", { progress: "p" }, { refresh: { event: "pageshow"; tag: "cw-card"; target: "window" } }>;
export type CopyButton = Setup<"a" | "button", {}, { copy: { event: "click"; tag: "a" } | { event: "click"; tag: "button" } }>;
export type CopyLink = Setup<"div", {}, { select: { event: "click"; tag: "input" } }>;
export type Crossword = Setup<"cw-crossword", { board: "div"; clue: ("button")[]; clueLabel: "span"; clueText: "span"; completion: ("section")[]; completionTime: "time"; entryPreview: "p"; hintButton?: "button"; intro: "section"; keyboard: "section"; menu: ("div")[]; options?: "section"; puzzleJson: "script"; rebus?: "button"; rebusKey: ("button")[]; timer: "time" }>;
export type DetailsLink = Setup<"a", {}, { open: { event: "click"; tag: "a" } }>;
export type HeroWatch = Setup<"header">;
export type LeaveLink = Setup<"a" | "button", {}, { leave: { event: "click"; tag: "a" } | { event: "click"; tag: "button" } }>;
export type MarkdownHighlight = Setup<"cw-markdown", { backdrop: "pre" }>;
export type PasswordToggle = Setup<"div", {}, { toggle: { event: "click"; tag: "button" } }>;
export type ThemeSelect = Setup<"form", {}, { change: { event: "change"; tag: "select" } }>;
