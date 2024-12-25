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
import {
  directionToArrow,
  expandArrows,
  expandNumbers,
  getKeypressesRequired,
  part1,
  part2,
} from "./index";
import { readInput } from "./readInput";

const numericKeypadGraph = createKeypadGraph(NUMERIC_KEYPAD, " ");
const arrowKeypadGraph = createKeypadGraph(ARROW_KEYPAD, " ");

describe("Floyd-Warshall", () => {
  test.each<[NumericKey, NumericKey, Edge<NumericKey>[]]>([
    ["5", "5", []],
    ["5", "6", [{ dir: RIGHT, val: "6" }]],
    ["5", "4", [{ dir: LEFT, val: "4" }]],
    [
      "5",
      "7",
      [
        { dir: UP, val: "8" },
        { dir: LEFT, val: "7" },
      ],
    ],
    ["0", "A", [{ dir: RIGHT, val: "A" }]],
    [
      "0",
      "3",
      [
        { dir: UP, val: "2" },
        { dir: RIGHT, val: "3" },
      ],
    ],
    [
      "A",
      "7",
      [
        { dir: UP, val: "3" },
        { dir: UP, val: "6" },
        { dir: UP, val: "9" },
        { dir: LEFT, val: "8" },
        { dir: LEFT, val: "7" },
      ],
    ],
  ] as const)(
    "should find shortest paths between all pairs of numeric keypad keys",
    (start, destination, expected) => {
      const paths = floydWarshall(numericKeypadGraph);

      const actual = paths.get(start)!.get(destination)!;

      expect(
        actual.map((path) => path.map((e) => directionToArrow(e.dir)).join("")),
        `Expected path ${start} to ${destination} to be ${expected
          .map((e) => e.val)
          .join("")}`
      ).toContain(expected.map((e) => directionToArrow(e.dir)).join(""));
    }
  );

  test.each<[ArrowKey, ArrowKey, Edge<ArrowKey>[]]>([
    ["A", "A", []],
    ["v", "^", [{ dir: UP, val: "^" }]],
    [
      "v",
      "A",
      [
        { dir: UP, val: "^" },
        { dir: RIGHT, val: "A" },
      ],
    ],
    [">", "A", [{ dir: UP, val: "A" }]],
  ])(
    "should find shortest paths between all pairs of arrow keypad keys",
    (start, destination, expected) => {
      const paths = floydWarshall(arrowKeypadGraph);

      const actual = paths.get(start)!.get(destination)!;

      expect(actual[0]).toStrictEqual(expected);
    }
  );
});

describe(expandNumbers.name, () => {
  test("expands numbers in the sequence", () => {
    const sequence = "029A".split("") as NumericKey[];

    const expanded = expandNumbers(sequence, floydWarshall(numericKeypadGraph));
    expect(expanded).toContain("<A^A^^>AvvvA");
  });
});

describe(expandArrows.name, () => {
  test("expands arrows in the sequence", () => {
    let sequence = "<A^A>^^AvvvA";
    const shortestPaths = floydWarshall(arrowKeypadGraph);

    let actual = expandArrows(sequence, shortestPaths);

    let expected = "v<<A>>^A<A>AvA^<AA>A<vAAA^>A";
    expect(actual.every((p) => p.length === expected.length)).toBe(true);
    // expect(actual).toContain(expected);

    actual = expandArrows("v<<A>>^A<A>AvA^<AA>A<vAAA^>A", shortestPaths);
    expected =
      "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A";

    expect(actual.every((p) => p.length === expected.length)).toBe(true);
    // expect(actual).toContain(expected);
  });
});

describe(part1.name, () => {
  test("solves the example", async () => {
    const input = await readInput("Example");

    expect(part1(input)).toBe(126384);
  });
});

describe(getKeypressesRequired.name, () => {
  test("depth 1", async () => {
    expect(
      getKeypressesRequired("<", "A", 1, floydWarshall(arrowKeypadGraph))
    ).toBe(8);
  });
});

describe(part2.name, () => {
  test("solves the example", async () => {
    const input = await readInput("Example");

    expect(part2(input, 2)).toBe(126384);
  });

  test("debugging part 2", () => {
    expect(part2("3A", 0)).toBe(3 * 4);

    const numericPaths = floydWarshall(numericKeypadGraph);
    const arrowPaths = floydWarshall(arrowKeypadGraph);

    const expandedNumbers = expandNumbers(["3", "A"], numericPaths)[0];

    let actual = part2("3A", 0);
    expect(actual).toBe(3 * 4 /* "^A vA" */);
    expect(actual).toBe(3 * expandedNumbers.length);

    let expanded = expandedNumbers;
    const expand = () => expandArrows(expanded, arrowPaths)[0];

    for (let i = 1; i < 10; i++) {
      expanded = expand();
      console.log(`--- i: ${i} --- expanded length: ${expanded.length}`);
      actual = part2("3A", i);
      expect(
        actual,
        `expected result to match the previous solution for i: ${i}`
      ).toBe(3 * expanded.length);
    }
  });
});
