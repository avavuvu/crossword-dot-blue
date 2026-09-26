import type { Game } from "./_game/game";

export class CwCrossword extends HTMLElement {
    game: Game | null = null;

    async connectedCallback(): Promise<void> {
        const { attach } = await import("./_attach");
        if (this.isConnected) this.game = attach(this);
    }
}

if (!customElements.get("cw-crossword")) {
    customElements.define("cw-crossword", CwCrossword);
}
