import { createKeypadGraph } from "./createKeypadGraph";
import { LEFT, DOWN, UP, type Direction } from "./directions";
import { floydWarshall, type ShortestPaths } from "./floydWarshall";
import {
  ARROW_KEYPAD,
  NUMERIC_KEYPAD,
  type ArrowKey,
  type NumericKey,
} from "./keypads";

export function solve(input: string) {
  const numericKeypadGraph = createKeypadGraph(NUMERIC_KEYPAD, " ");
  const arrowKeypadGraph = createKeypadGraph(ARROW_KEYPAD, " ");

  const numericKeypadShortestPaths = floydWarshall(numericKeypadGraph);
  const arrowKeypadShortestPaths = floydWarshall(arrowKeypadGraph);

  const sequences = input.split("\n");

  const ARROW_NESTING_LEVELS = 2;

  const expanded = sequences.map((sequence) => {
    let arrows = expandNumbers(
      sequence.split("") as NumericKey[],
      numericKeypadShortestPaths
    );

    for (let nesting = 0; nesting < ARROW_NESTING_LEVELS; nesting++) {
      arrows = expandArrows(arrows, arrowKeypadShortestPaths);
    }

    return arrows;
  });

  const complexities = sequences.map((sequence, i) => {
    const numericPartOfTheCode = parseInt(sequence);

    if (!numericPartOfTheCode) {
      throw new Error("No numeric part of the code found");
    }

    return expanded[i].length * numericPartOfTheCode;
  });

  console.log(
    complexities
      .map(
        (complexity, i) =>
          `${sequences[i]}: ${expanded[i].length} * ${parseInt(
            sequences[i]
          )} == ${complexity}`
      )
      .join("\n")
  );
  return complexities.reduce((acc, complexity) => acc + complexity, 0);
}

export function expandNumbers(
  keys: NumericKey[],
  paths: ShortestPaths<NumericKey>
): ArrowKey[] {
  const start = "A" as NumericKey;

  let res: ArrowKey[] = [];

  let current = start;
  for (const key of keys) {
    const toKey = paths.get(current)!.get(key)!;

    res.push(...toKey.map((edge) => directionToArrow(edge.dir)), "A");

    current = key;
  }

  return res;
}

export function expandArrows(
  keys: ArrowKey[],
  arrowKeypadShortestPaths: ShortestPaths<ArrowKey>
): ArrowKey[] {
  const start = "A" as ArrowKey;

  let res: ArrowKey[] = [];

  let current = start;
  for (const key of keys) {
    const toKey = arrowKeypadShortestPaths.get(current)!.get(key)!;

    res.push(...toKey.map((edge) => directionToArrow(edge.dir)), "A");

    current = key;
  }

  return res;
}

function directionToArrow(dir: Direction): ArrowKey {
  return dir === UP ? "^" : dir === DOWN ? "v" : dir === LEFT ? "<" : ">";
}
