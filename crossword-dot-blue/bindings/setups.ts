import type { Setup } from "@bq/components/src/setups";

export type CardProgress = Setup<"cw-card", { progress: "p" }>;
export type Crossword = Setup<"cw-crossword", { board: "div"; clue: ("button")[]; clueLabel: "span"; clueText: "span"; completion: ("section")[]; completionTime: "time"; entryPreview: "p"; hintButton?: "button"; intro: "section"; keyboard: "section"; menu: ("div")[]; options?: "section"; puzzleJson: "script"; rebus?: "button"; rebusKey: ("button")[]; timer: "time" }>;
export type HeroWatch = Setup<"header">;
export type MarkdownHighlight = Setup<"cw-markdown", { backdrop: "pre" }>;
export type PasswordToggle = Setup<"div", { toggle: "button" }>;
export type ThemeSelect = Setup<"form", { select: "select" }>;
