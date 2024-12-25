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

// #region part 1

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

export function part1(input: string) {
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

// #endregion part 1

// #region part 2

const getKeypressesRequired = memoize(function getKeypressesRequiredInternal<
  TKey
>(from: TKey, to: TKey, depth: number, shortestPaths: ShortestPaths<TKey>) {
  const paths = shortestPaths.get(from)!.get(to)!;

  if (depth === 0) {
    return paths[0].length;
  }

  let leastKeypressesRequired = Infinity;

  for (const path of paths) {
    const sequence = [
      "A" as TKey, // we always start with A
      ...path.map((edge) => directionToArrow(edge.dir)),
    ];
    let cost = 0;
    for (let i = 0; i < sequence.length; i++) {
      const key = sequence[i];
      const nextKey = sequence[i + 1];

      if (nextKey) {
        cost += getKeypressesRequired(key, nextKey, depth - 1, shortestPaths);
      }
    }

    if (cost < leastKeypressesRequired) {
      leastKeypressesRequired = cost;
    }
  }

  return leastKeypressesRequired;
});

export function part2(input: string, arrowNestingLevels: number = 25) {
  const numericKeypadGraph = createKeypadGraph(NUMERIC_KEYPAD, " ");
  const arrowKeypadGraph = createKeypadGraph(ARROW_KEYPAD, " ");

  const numericKeypadShortestPaths = floydWarshall(numericKeypadGraph);
  const arrowKeypadShortestPaths = floydWarshall(arrowKeypadGraph);

  const sequences = input.split("\n");

  const keypressesRequired = sequences.map((sequence) => {
    let arrows = expandNumbers(
      sequence.split("") as NumericKey[],
      numericKeypadShortestPaths
    );

    const keypressesRequired = arrows.map((keys) => {
      const sequence = "A" + keys;
      let cost = 0;
      for (let i = 0; i < sequence.length; i++) {
        const key = sequence[i];
        const nextKey = sequence[i + 1];
        if (nextKey) {
          cost += getKeypressesRequired(
            key,
            nextKey,
            arrowNestingLevels,
            arrowKeypadShortestPaths
          );
        }
      }
      return cost;
    });

    return keypressesRequired.reduce(
      (acc, cost) => (acc < cost ? acc : cost),
      Infinity
    );
  });

  const complexities = sequences.map((sequence, i) => {
    const numericPartOfTheCode = parseInt(sequence);
    const keypresses = keypressesRequired[i];
    return keypresses * numericPartOfTheCode;
  });

  return complexities.reduce((acc, complexity) => acc + complexity, 0);
}

function memoize<F extends (...args: any[]) => any>(fn: F): F {
  const cache = new Map<string, ReturnType<F>>();
  return ((...args: Parameters<F>) => {
    const key = JSON.stringify(args);
    if (cache.has(key)) return cache.get(key)!;
    const result = fn(...args);
    cache.set(key, result);
    return result;
  }) as F;
}

// #endregion part 2

if (import.meta.main) {
  const input = await readInput("Input");

  if (process.argv.includes("--part=1")) {
    console.log(part1(input));
  } else if (process.argv.includes("--part=2")) {
    console.log(part2(input));
  } else {
    console.log("Please specify --part=1 or --part=2");
  }
}
