import type { Registry } from "@bq/components/src/setups";
import type * as setups from "./setups";
import { cardProgress } from "../src/components/card/index";

export const registry: Registry = {
    "card-progress": {
        tags: ["cw-card"],
        many: [],
        setup: cardProgress satisfies setups.CardProgress,
    },
    "crossword": {
        tags: ["cw-crossword"],
        many: ["clue", "completion", "menu", "rebusKey"],
        load: () => import("../src/components/crossword/index").then((module) => module.crossword satisfies setups.Crossword),
    },
    "hero-watch": {
        tags: ["header"],
        many: [],
        load: () => import("../src/components/header/index").then((module) => module.heroWatch satisfies setups.HeroWatch),
    },
    "markdown-highlight": {
        tags: ["cw-markdown"],
        many: [],
        load: () => import("../src/components/markdown/index").then((module) => module.markdownHighlight satisfies setups.MarkdownHighlight),
    },
    "password-toggle": {
        tags: ["div"],
        many: [],
        load: () => import("@bq/components/src/input/index").then((module) => module.passwordToggle satisfies setups.PasswordToggle),
    },
    "theme-select": {
        tags: ["form"],
        many: [],
        load: () => import("../src/components/options/index").then((module) => module.themeSelect satisfies setups.ThemeSelect),
    },
};
