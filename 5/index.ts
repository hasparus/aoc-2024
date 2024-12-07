import { readFileSync } from "fs";

type PageNumber = number;
type Update = PageNumber[];

function parse(input: string) {
  const [rules, updates] = input.trim().split(/\n\s*\n/);

  const isBeforeMap = new Map<PageNumber, PageNumber[]>();

  for (const rule of rules.split("\n")) {
    const [predecessor, successor] = rule.split("|").map(Number);
    const before = isBeforeMap.get(predecessor) ?? [];
    before.push(successor);
    isBeforeMap.set(predecessor, before);
  }

  return {
    updates: updates
      .trim()
      .split("\n")
      .map((line) => line.trim())
      .filter((line) => line.length)
      .map((line) => line.split(",").map(Number)),
    /**
     * page a must be printed before pages b
     */
    isBefore: (a: number, b: number) =>
      isBeforeMap.get(a)?.includes(b) ?? false,
  };
}

function isValidUpdate(
  update: Update,
  isBefore: (a: number, b: number) => boolean
) {
  for (let i = update.length - 1; i > 0; i--) {
    if (isBefore(update[i], update[i - 1])) {
      return false;
    }
  }
  return true;
}

function sumMiddleNumbers(updates: Update[]) {
  return updates.reduce((sum, update) => {
    const middleNumber = update[Math.floor(update.length / 2)];
    return sum + middleNumber;
  }, 0);
}

function assert(condition: unknown, message: string): asserts condition {
  if (!condition) throw new Error(message);
}

function solve1(input: string) {
  const { updates, isBefore } = parse(input);
  const valid = updates.filter((update) => isValidUpdate(update, isBefore));
  return sumMiddleNumbers(valid);
}

function solve2(input: string) {
  const { updates, isBefore } = parse(input);
  const invalid = updates.filter((update) => !isValidUpdate(update, isBefore));

  const fixed = invalid.map((update) => {
    return update.sort((a, b) => (isBefore(a, b) ? -1 : 1));
  });

  return sumMiddleNumbers(fixed);
}

const input = readFileSync(new URL("./input.txt", import.meta.url), "utf-8");

console.log(solve1(input));
console.log(solve2(input));

if (process.argv.slice(2).includes("test")) {
  const EXAMPLE = readFileSync(
    new URL("./example.txt", import.meta.url),
    "utf-8"
  );

  assert(solve1(EXAMPLE) === 143, "example: filtering out invalid");
  assert(solve2(EXAMPLE) === 123, "example: fixing invalid");
}
