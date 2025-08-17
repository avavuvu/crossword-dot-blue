import type { GameStateStore } from "./gameState.svelte"

export interface TimeManagerInterface {
    reset(restartTimer: boolean): void

    start(): void
    pause(): void
    resume(): void

    get timeInSeconds(): number
    get formattedTime(): string
}

export class TimeManager implements TimeManagerInterface {
    state: GameStateStore
    isRunning = false
    #startTime = 0
    #pausedTime = 0

    constructor(state: GameStateStore) {
        this.state = state
    }

    start() {
        if (!this.isRunning) {
            this.#startTime = performance.now() - this.state.elapsedTime;
            this.isRunning = true;
            this.run();
        }
    }

    pause() {
        if (this.isRunning) {
            this.isRunning = false;
            this.#pausedTime = performance.now();
        }
    }

    resume() {
        if (!this.isRunning) {
            this.#startTime = performance.now() - (this.#pausedTime - this.#startTime);
            this.isRunning = true;
            this.run();
        }
    }

    reset(restartTimer: boolean): void {
        this.#pausedTime = 0;
        this.state.elapsedTime = 0;
        this.#startTime = performance.now() - this.state.elapsedTime;

        if(!restartTimer) {
            this.isRunning = false
        }
    }

    getElapsedTime() {
        if (this.isRunning) {
            return performance.now() - this.#startTime;
        } else {
            return this.state.elapsedTime;
        }
    }

    get timeInSeconds() {
        return Math.floor((this.state.elapsedTime) / 1000)
    }

    get formattedTime() {
        const pad = (str: string) => str.padStart(2,'0')

        const inSeconds = Math.floor((this.state.elapsedTime) / 1000)
        const minutes = (Math.floor(inSeconds / 60)).toString()
        const seconds = (inSeconds % 60).toString()

        return `${pad(minutes)}:${pad(seconds)}`
    }

    run() {
        if (this.isRunning) {
            this.state.elapsedTime = performance.now() - this.#startTime;
            requestAnimationFrame(() => this.run())

        }
    }
}