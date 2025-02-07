import { gameManager } from "./gameManager.svelte";

export const inputKey = (event: KeyboardEvent) => {
    switch(event.key) {
        case "Tab": 
            gameManager.clueManager.cycleClues(
                event.getModifierState("Shift") ? -1 : 1
            )
        break;
        case "Enter": 
            gameManager.clueManager.cycleClues(1)
        break;
        case "Backspace":
            gameManager.gridManager.backspace()
        break;
        case " ":
            gameManager.clueManager.toggleOrientation()
        break;
        case "ArrowLeft":
            gameManager.gridManager.moveBy([-1, 0])
        break;
        case "ArrowRight":
            gameManager.gridManager.moveBy([1, 0])
        break;
        case "ArrowUp":
            gameManager.gridManager.moveBy([0, -1])
        break;
        case "ArrowDown":
            gameManager.gridManager.moveBy([0, 1])  
        break;
        case "`":
            gameManager.state.isRebusMode = !gameManager.state.isRebusMode
        break;
        case "Rebus":
            gameManager.state.isRebusMode = !gameManager.state.isRebusMode
        break;
        case "Toggle":
            gameManager.clueManager.toggleOrientation()
        break;
        default:
            if (event.key.match(/^[a-zA-Z]$/)) {
                gameManager.gridManager.inputChar(event.key.toLowerCase())
            }
        break;
    }

}
