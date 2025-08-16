import type { Orientation } from "../game/types"

export type Coord = [number, number]

export const addCoord = (a: Coord, b: Coord): Coord => [a[0] + b[0], a[1] + b[1]]

export const coordsToIndex = ([x, y]: Coord, width: number): number => {
    if (x < 0 || y < 0) {
        throw new Error('Coordinates must be non-negative');
    }
    return y * width + x;
}

export const indexToCoords = (index: number, width: number): Coord => {
    if (index < 0) {
        throw new Error('Index must be non-negative');
    }
    return [
        index % width,
        Math.floor(index / width)
    ]
}

export const getOrientation = (isHorizontal: boolean): Orientation => {
    return isHorizontal ? 'across' : 'down'
}