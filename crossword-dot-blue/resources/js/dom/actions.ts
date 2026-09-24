import type { Game } from "../game/game";
import { actionFromNames } from "./action-map";

export type ActionsUi = {
    start(): void;
    dismiss(): void;
};

export function attachActions(game: Game, root: HTMLElement, ui: ActionsUi): void {
    root.addEventListener("click", (event) => {
        const button = (event.target as Element).closest<HTMLElement>("[data-action]");
        if (!button || !root.contains(button)) return;

        const name = button.dataset.action!;
        button.closest<HTMLElement>("[popover]")?.hidePopover();

        const actions = actionFromNames(name);
        for (const action of actions) {
            switch (action.type) {
                case "start":
                    ui.start();
                    return;
                case "dismiss":
                    ui.dismiss();
                    return;
                default:
                    game.dispatch(action);
                    break;
            }

        }
    });
}
