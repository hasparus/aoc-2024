import { createKeypadGraph } from "./createKeypadGraph";
import { LEFT, DOWN, UP, type Direction } from "./directions";
import { floydWarshall, type ShortestPaths } from "./floydWarshall";
import {
  ARROW_KEYPAD,
  NUMERIC_KEYPAD,
  type ArrowKey,
  type NumericKey,
} from "./keypads";
import { readInput } from "./readInput";

function keepShortestPaths(paths: string[]): string[] {
  if (paths.length === 0) return paths;

  let minLength = Infinity;
  for (const path of paths) {
    const length = path.length;
    if (length < minLength) {
      minLength = length;
    }
  }

  const shortestPaths = paths.filter((p) => p.length === minLength);

  return Array.from(new Set(shortestPaths));
}

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
      arrows = arrows.flatMap((path) =>
        expandArrows(path, arrowKeypadShortestPaths)
      );
      arrows = keepShortestPaths(arrows);
    }

    return arrows[0];
  });

  const complexities = sequences.map((sequence, i) => {
    const numericPartOfTheCode = parseInt(sequence);

    if (!numericPartOfTheCode) {
      throw new Error("No numeric part of the code found");
    }

    return expanded[i].length * numericPartOfTheCode;
  });

  return complexities.reduce((acc, complexity) => acc + complexity, 0);
}

export function expandNumbers(
  keys: NumericKey[],
  paths: ShortestPaths<NumericKey>
): string[] {
  const start = "A" as NumericKey;

  let allPaths = [""];

  let current = start;
  for (const key of keys) {
    const possiblePaths = paths.get(current)!.get(key)!;

    allPaths = allPaths.flatMap((path) =>
      possiblePaths.map(
        (edges) =>
          path + edges.map((edge) => directionToArrow(edge.dir)).join("") + "A"
      )
    );

    allPaths = keepShortestPaths(allPaths);
    current = key;
  }

  return allPaths;
}

export function expandArrows(
  path: string,
  arrowKeypadShortestPaths: ShortestPaths<ArrowKey>
): string[] {
  const start = "A" as ArrowKey;

  let allPaths = [""];

  let current = start;
  for (const key of path) {
    const possiblePaths = arrowKeypadShortestPaths
      .get(current)!
      .get(key as ArrowKey)!;

    allPaths = allPaths.flatMap((p) =>
      possiblePaths.map(
        (edges) =>
          p + edges.map((edge) => directionToArrow(edge.dir)).join("") + "A"
      )
    );

    allPaths = keepShortestPaths(allPaths);
    current = key as ArrowKey;
  }

  return allPaths;
}

export function directionToArrow(dir: Direction): ArrowKey {
  return dir === UP ? "^" : dir === DOWN ? "v" : dir === LEFT ? "<" : ">";
}

if (import.meta.main) {
  const input = await readInput("Input");
  console.log(solve(input));
}
