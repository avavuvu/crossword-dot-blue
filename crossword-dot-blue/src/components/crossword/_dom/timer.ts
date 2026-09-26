import type { Game } from "../_game/game";
import { formatDuration, isoDuration } from "../_game/time";

export type Timer = {
    elapsed(): number;
    setElapsed(ms: number): void;
    start(): void;
};

export function attachTimer(game: Game, root: HTMLElement): Timer {
    const display = root.querySelector<HTMLElement>("[data-timer]");
    const finalDisplay = root.querySelector<HTMLElement>("[data-completion-time]");

    let base = 0;
    let runningSince: number | null = null;
    let started = false;
    let finished = game.state.completion === "won";
    let interval: ReturnType<typeof setInterval> | undefined;

    const elapsed = () => base + (runningSince === null ? 0 : performance.now() - runningSince);

    const paint = () => {
        const ms = elapsed();
        const text = formatDuration(ms);
        if (display) {
            display.textContent = text;
            display.setAttribute("datetime", isoDuration(ms));
        }
        if (finalDisplay && finished) {
            finalDisplay.textContent = text;
            finalDisplay.setAttribute("datetime", isoDuration(ms));
        }
    };

    const shouldRun = () =>
        started && !finished && document.visibilityState === "visible" && document.hasFocus();

    const update = () => {
        const run = shouldRun();

        if (run && runningSince === null) {
            runningSince = performance.now();
            interval = setInterval(paint, 500);
        } else if (!run && runningSince !== null) {
            base += performance.now() - runningSince;
            runningSince = null;
            clearInterval(interval);
            interval = undefined;
        }

        paint();
    };

    document.addEventListener("visibilitychange", update);
    window.addEventListener("focus", update);
    window.addEventListener("blur", update);

    const start = () => {
        started = true;
        update();
    };

    game.on("completion", ({ state }) => {
        finished = state === "won";
        update();
    });

    game.on("reset", () => {
        base = 0;
        runningSince = null;
        finished = false;
        update();
    });

    game.on("restore", () => {
        finished = game.state.completion === "won";
        update();
    });

    paint();

    return {
        elapsed,
        setElapsed(ms) {
            base = ms;
            if (runningSince !== null) runningSince = performance.now();
            paint();
        },
        start,
    };
}
