import type { Registry } from "./boutique/setups";
import type * as setups from "./setups";
import { autosaveForm } from "../src/components/autosave/index";
import { cardProgress } from "../src/components/card/index";
import { leaveLink } from "../src/components/autosave/index";

export const registry: Registry = {
    "autosave-form": {
        tags: ["form"],
        many: [],
        setup: autosaveForm satisfies setups.AutosaveForm,
    },
    "card-progress": {
        tags: ["cw-card"],
        many: [],
        setup: cardProgress satisfies setups.CardProgress,
    },
    "copy-button": {
        tags: ["a", "button"],
        many: [],
        load: () => import("../src/components/copy/index").then((module) => module.copyButton satisfies setups.CopyButton),
    },
    "copy-link": {
        tags: ["div"],
        many: [],
        load: () => import("../src/components/copy/index").then((module) => module.copyLink satisfies setups.CopyLink),
    },
    "crossword": {
        tags: ["cw-crossword"],
        many: ["clue", "completion", "menu", "rebusKey"],
        load: () => import("../src/components/crossword/index").then((module) => module.crossword satisfies setups.Crossword),
    },
    "details-link": {
        tags: ["a"],
        many: [],
        load: () => import("../src/components/details_link/index").then((module) => module.detailsLink satisfies setups.DetailsLink),
    },
    "hero-watch": {
        tags: ["header"],
        many: [],
        load: () => import("../src/components/header/index").then((module) => module.heroWatch satisfies setups.HeroWatch),
    },
    "leave-link": {
        tags: ["a", "button"],
        many: [],
        setup: leaveLink satisfies setups.LeaveLink,
    },
    "markdown-highlight": {
        tags: ["cw-markdown"],
        many: [],
        load: () => import("../src/components/markdown/index").then((module) => module.markdownHighlight satisfies setups.MarkdownHighlight),
    },
    "password-toggle": {
        tags: ["div"],
        many: [],
        load: () => import("./boutique/input/index").then((module) => module.passwordToggle satisfies setups.PasswordToggle),
    },
    "theme-select": {
        tags: ["form"],
        many: [],
        load: () => import("../src/components/options/index").then((module) => module.themeSelect satisfies setups.ThemeSelect),
    },
};
