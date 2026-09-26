import type { Clue } from "@bindings/Clue";
import type { Direction } from "@bindings/Direction";
import type { Puzzle } from "@bindings/Puzzle";

export function isBlock(puzzle: Puzzle, index: number): boolean {
    return puzzle.cells[index]?.type !== "letter";
}

export function solutionAt(puzzle: Puzzle, index: number): string | null {
    const cell = puzzle.cells[index];
    return cell?.type === "letter" ? cell.solution : null;
}

export function clueIdAt(puzzle: Puzzle, index: number, direction: Direction): string | null {
    const cell = puzzle.cells[index];
    return cell?.type === "letter" ? cell.clues[direction] : null;
}

export function clueAt(puzzle: Puzzle, index: number, direction: Direction): Clue | null {
    const id = clueIdAt(puzzle, index, direction);
    return id ? puzzle.clues[id] ?? null : null;
}

export function opposite(direction: Direction): Direction {
    return direction === "across" ? "down" : "across";
}

export function orderedClues(puzzle: Puzzle): Clue[] {
    return puzzle.clueOrder.map((id) => puzzle.clues[id]!);
}

export function nextClue(puzzle: Puzzle, id: string, delta: 1 | -1): Clue {
    const order = puzzle.clueOrder;
    const position = order.indexOf(id);
    const next = (position + delta + order.length) % order.length;
    return puzzle.clues[order[next]!]!;
}

export function letterIndexes(puzzle: Puzzle): number[] {
    const indexes: number[] = [];
    puzzle.cells.forEach((cell, index) => {
        if (cell.type === "letter") indexes.push(index);
    });
    return indexes;
}

export function matches(puzzle: Puzzle, index: number, entry: string): boolean {
    const solution = solutionAt(puzzle, index);
    return solution !== null && solution.toUpperCase() === entry.toUpperCase();
}
