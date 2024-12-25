export const UP = [-1, 0] as const;
export const DOWN = [1, 0] as const;
export const LEFT = [0, -1] as const;
export const RIGHT = [0, 1] as const;

export const DIRECTIONS = [UP, RIGHT, DOWN, LEFT];

export type Direction = (typeof DIRECTIONS)[number];
