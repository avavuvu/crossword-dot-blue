import type { Game } from "../game/game";
import { actionsFromAttribute } from "./action-map";

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

        for (const action of actionsFromAttribute(name)) {
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
