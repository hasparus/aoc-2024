export const NUMERIC_KEYPAD = [
  ["7", "8", "9"],
  ["4", "5", "6"],
  ["1", "2", "3"],
  [" ", "0", "A"],
] as const;

export type NumericKey = (typeof NUMERIC_KEYPAD)[number][number];

export const ARROW_KEYPAD = [
  [" ", "^", "A"],
  ["<", "v", ">"],
] as const;

export type ArrowKey = (typeof ARROW_KEYPAD)[number][number];
