type Heights = number[];
const FULL_HEIGHT = 7;

type Key = { type: "key"; heights: Heights };
type Lock = { type: "lock"; heights: Heights };

function convertSchematicToPinHeights(schematic: string): Key | Lock {
  let heights = [0, 0, 0, 0, 0];
  const lines = schematic.split("\n");

  for (const line of lines) {
    for (let i = 0; i < line.length; i++) {
      if (line[i] === "#") {
        heights[i]++;
      } else if (line[i] !== ".") {
        throw new Error("Invalid schematic");
      }
    }
  }

  return lines[0] === "#####"
    ? { type: "key", heights }
    : { type: "lock", heights };
}

function parseInput(input: string) {
  input = input.trim();

  const keys: Heights[] = [];
  const locks: Heights[] = [];

  for (const schematic of input.split("\n\n")) {
    const { type, heights } = convertSchematicToPinHeights(schematic);
    if (type === "key") {
      keys.push(heights);
    } else {
      locks.push(heights);
    }
  }

  return { locks, keys };
}

function canFit(lock: Heights, key: Heights) {
  return lock.every((height, i) => height + key[i] <= FULL_HEIGHT);
}

function solve(input: string) {
  const { locks, keys } = parseInput(input);

  let result = 0;

  for (const lock of locks) {
    for (const key of keys) {
      if (canFit(lock, key)) {
        result++;
      }
    }
  }

  return result;
}

if (import.meta.main) {
  const input = await Bun.file("input.txt").text();
  console.log(solve(input));
}
