import { expect, test, describe } from "bun:test";
import { floydWarshall } from "./floydWarshall";
import {
  ARROW_KEYPAD,
  NUMERIC_KEYPAD,
  type ArrowKey,
  type NumericKey,
} from "./keypads";
import { createKeypadGraph } from "./createKeypadGraph";
import type { Edge } from "./graph";
import { LEFT, RIGHT, UP } from "./directions";
import { expandArrows, expandNumbers, solve } from "./index";
import { readInput } from "./readInput";

const numericKeypadGraph = createKeypadGraph(NUMERIC_KEYPAD, " ");
const arrowKeypadGraph = createKeypadGraph(ARROW_KEYPAD, " ");

describe("Floyd-Warshall", () => {
  test.each<[NumericKey, NumericKey, Edge<NumericKey>[]]>([
    ["5", "5", []],
    ["5", "6", [{ dir: RIGHT, val: "6", weight: 1 }]],
    ["5", "4", [{ dir: LEFT, val: "4", weight: 1 }]],
    [
      "5",
      "7",
      [
        { dir: UP, val: "8", weight: 1 },
        { dir: LEFT, val: "7", weight: 1 },
      ],
    ],
    ["0", "A", [{ dir: RIGHT, val: "A", weight: 1 }]],
    [
      "0",
      "3",
      [
        { dir: UP, val: "2", weight: 1 },
        { dir: RIGHT, val: "3", weight: 1 },
      ],
    ],
    [
      "A",
      "7",
      [
        { dir: UP, val: "3", weight: 1 },
        { dir: UP, val: "6", weight: 0.9 },
        { dir: UP, val: "9", weight: 0.9 },
        { dir: LEFT, val: "8", weight: 1 },
        { dir: LEFT, val: "7", weight: 0.9 },
      ],
    ],
  ] as const)(
    "should find shortest paths between all pairs of numeric keypad keys",
    (start, destination, expected) => {
      const paths = floydWarshall(numericKeypadGraph);

      const actual = paths.get(start)!.get(destination)!;

      expect(
        actual,
        `Expected path ${start} to ${destination} to be ${expected
          .map((e) => e.val)
          .join("")}`
      ).toStrictEqual(expected);
    }
  );

  test.each<[ArrowKey, ArrowKey, Edge<ArrowKey>[]]>([
    ["A", "A", []],
    ["v", "^", [{ dir: UP, val: "^", weight: 1 }]],
    [
      "v",
      "A",
      [
        { dir: UP, val: "^", weight: 1 },
        { dir: RIGHT, val: "A", weight: 1 },
      ],
    ],
    [">", "A", [{ dir: UP, val: "A", weight: 1 }]],
  ])(
    "should find shortest paths between all pairs of arrow keypad keys",
    (start, destination, expected) => {
      const paths = floydWarshall(arrowKeypadGraph);

      const actual = paths.get(start)!.get(destination)!;

      expect(actual).toStrictEqual(expected);
    }
  );
});

describe(expandNumbers.name, () => {
  test("expands numbers in the sequence", () => {
    const sequence = "029A".split("") as NumericKey[];

    const expanded = expandNumbers(sequence, floydWarshall(numericKeypadGraph));
    expect(expanded.join("")).toBe("<A^A^^>AvvvA");
  });
});

describe(expandArrows.name, () => {
  test("expands arrows in the sequence", () => {
    let sequence = "<A^A>^^AvvvA";
    const shortestPaths = floydWarshall(arrowKeypadGraph);

    let actual = expandArrows(sequence.split("") as ArrowKey[], shortestPaths);

    expect(actual.join("")).toBe("v<<A>>^A<A>AvA^<AA>A<vAAA^>A");

    actual = expandArrows(actual, shortestPaths);

    expect(actual.join("").length).toBe(
      "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"
        .length
    );
  });
});

describe(solve.name, () => {
  test("solves the example", async () => {
    const input = await readInput("Example");

    expect(solve(input)).toBe(126384);
  });

  test("solves the example: problem 1", () => {
    const sequence = "379A";

    const numericPaths = floydWarshall(numericKeypadGraph);
    let step1 = expandNumbers(sequence.split("") as NumericKey[], numericPaths);

    let arrowPaths = floydWarshall(arrowKeypadGraph);
    const step2 = expandArrows(step1, arrowPaths);
    const step3 = expandArrows(step2, arrowPaths);

    expect(step3.join("")).toBe(
      "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"
    );
  });
});
