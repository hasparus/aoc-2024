import { readFileSync } from "fs";

const OBSTACLE = "#";
const EMPTY = ".";
const VISITED_HORIZONTAL = "-";
const VISITED_VERTICAL = "|";
const VISITED_BOTH = "+";

type Cell =
  | typeof OBSTACLE
  | typeof EMPTY
  | typeof VISITED_HORIZONTAL
  | typeof VISITED_VERTICAL
  | typeof VISITED_BOTH
  | GuardDirection;

function isVisited(cell: Cell) {
  return (
    cell === VISITED_HORIZONTAL ||
    cell === VISITED_VERTICAL ||
    cell === VISITED_BOTH
  );
}

type Row = Cell[];
type Map = Row[];

const parseInput = (input: string): Map => {
  return input
    .trim()
    .split("\n")
    .map((line) => line.trim().split("") as Row);
};

function getCell(map: Map, position: Position) {
  return map[position[1]][position[0]];
}

function setCell(map: Map, position: Position, value: Cell) {
  map[position[1]][position[0]] = value;
}

function stringifyMap(map: Map, guard?: Guard): string {
  if (!guard) {
    return map.map((row) => row.join("")).join("\n");
  }

  const { position, direction } = guard;
  return map
    .map((row, y) =>
      row
        .map((cell, x) =>
          x === position[0] && y === position[1] ? direction : cell
        )
        .join("")
    )
    .join("\n");
}

/**
 * Adds column numbers and row numbers for easier debugging
 */
function decorateMap(map: string): string {
  const lines = map.split("\n");
  const maxWidth = Math.max(...lines.map((line) => line.length));
  const columnNumbers = Array.from({ length: maxWidth }, (_, i) => i % 10).join(
    ""
  );

  return [`  ${columnNumbers}`, ...lines.map((line, y) => `${y} ${line}`)].join(
    "\n"
  );
}

function cloneMap(map: Map): Map {
  return map.map((row) => [...row]);
}

type Position = [x: number, y: number];

function add(a: Position, b: Position): Position {
  return [a[0] + b[0], a[1] + b[1]];
}

const UP: Position = [0, -1];
const RIGHT: Position = [1, 0];
const DOWN: Position = [0, 1];
const LEFT: Position = [-1, 0];

const MOVEMENTS = { "^": UP, ">": RIGHT, v: DOWN, "<": LEFT };

type GuardDirection = keyof typeof MOVEMENTS;

const DIRECTION_SYMBOLS = Object.keys(MOVEMENTS) as GuardDirection[];

function rotateRight(direction: GuardDirection): GuardDirection {
  return DIRECTION_SYMBOLS[(DIRECTION_SYMBOLS.indexOf(direction) + 1) % 4];
}

interface Guard {
  position: Position;
  direction: GuardDirection;
}

function findGuard(map: Map) {
  for (let y = 0; y < map.length; y++) {
    for (let x = 0; x < map[y].length; x++) {
      const pos: Position = [x, y];
      const cell = getCell(map, pos);
      if ((DIRECTION_SYMBOLS as string[]).includes(cell)) {
        return {
          position: pos,
          direction: cell as GuardDirection,
        };
      }
    }
  }

  return null;
}

const VERBOSE =
  process.env.VERBOSE === "true" || process.argv.includes("--verbose");
const ANIMATE = process.argv.includes("--animate");

async function playAndMarkTrails(map: Map) {
  let guard = findGuard(map);

  if (!guard) {
    console.error("Guard not found");
    console.error(stringifyMap(map));
    throw new Error("Guard not found");
  }

  let time = 0;

  while (true) {
    // move the guard and update map
    {
      const forward = add(guard.position, MOVEMENTS[guard.direction]);
      const guardWalkedOutOfMap =
        forward[1] < 0 ||
        forward[1] >= map.length ||
        forward[0] < 0 ||
        forward[0] >= map[0].length;

      const verticalMovement =
        guard.direction === "v" || guard.direction === "^";
      const currentCell = getCell(map, guard.position);

      let trail: Cell = ".";
      if (isVisited(currentCell)) {
        if (currentCell === VISITED_HORIZONTAL && verticalMovement) {
          trail = VISITED_BOTH;
        } else if (currentCell === VISITED_VERTICAL && !verticalMovement) {
          trail = VISITED_BOTH;
        }
      } else if (verticalMovement) {
        trail = VISITED_VERTICAL;
      } else {
        trail = VISITED_HORIZONTAL;
      }

      if (guardWalkedOutOfMap) {
        setCell(map, guard.position, trail);
        break;
      }

      if (getCell(map, forward) === OBSTACLE) {
        setCell(map, guard.position, VISITED_BOTH);
        guard.direction = rotateRight(guard.direction);
        guard.position = add(guard.position, MOVEMENTS[guard.direction]);
      } else {
        setCell(map, guard.position, trail);
        guard.position = forward;
      }
    }

    time++;

    if (VERBOSE) {
      if (ANIMATE) console.clear();
      console.log(stringifyMap(map, guard));
      console.log("Time:", time);
      if (ANIMATE) await new Promise((resolve) => setTimeout(resolve, 25));
    }
  }
}

const example = readFileSync(
  new URL("./example.txt", import.meta.url),
  "utf-8"
);

const input = readFileSync(new URL("./input.txt", import.meta.url), "utf-8");

if (process.argv.includes("--part=1")) {
  const map = parseInput(example);
  playAndMarkTrails(map);
  console.log(
    "1 // example: Visited positions:",
    map.flat().filter(isVisited).length
  );

  await playAndMarkTrails(parseInput(input));
}

function putObstaclesAndSquaresAndCountCycles(map: Map) {
  const originalGuard = findGuard(map);
  const visitedMap = cloneMap(map);
  playAndMarkTrails(visitedMap);

  if (!originalGuard) {
    throw new Error("Guard not found in putObstaclesAndSquaresAndCountCycles");
  }

  // console.log(decorateMap(stringifyMap(visitedMap, originalGuard)));
  // console.log("(3,6) is", getCell(visitedMap, [3, 6]));
  // console.log("guard is", originalGuard);

  const visitedPositions = visitedMap
    .flatMap((row, y) =>
      row.map((cell, x): Position | null => (isVisited(cell) ? [x, y] : null))
    )
    .filter(
      (x): x is Position =>
        x !== null &&
        !(
          x[0] === originalGuard.position[0] &&
          x[1] === originalGuard.position[1]
        )
    );

  let positionsWithObstaclesCreatingACycle = 0;

  for (const position of visitedPositions) {
    const newMap = cloneMap(map);
    setCell(newMap, position, OBSTACLE);
    if (detectCycle(newMap)) {
      // console.log("Cycle detected at", position);
      // setCell(newMap, position, "O" as Cell);
      // console.log(stringifyMap(newMap));
      positionsWithObstaclesCreatingACycle++;
    }
  }

  console.log(
    "\n2 // example: Positions with obstacles creating a cycle:",
    positionsWithObstaclesCreatingACycle
  );
}

function detectCycle(map: Map): boolean {
  let guard = findGuard(map);

  if (!guard) {
    console.error("Guard not found");
    console.error(stringifyMap(map));
    throw new Error("Guard not found");
  }

  let time = 0;

  // if a guard hits the same obstacle twice, we have a cycle
  type Hit = `${Position[0]}-${Position[1]}-${GuardDirection}`;
  const hits: Set<Hit> = new Set();

  while (true) {
    // move the guard and update map
    {
      const forward = add(guard.position, MOVEMENTS[guard.direction]);
      const guardWalkedOutOfMap =
        forward[1] < 0 ||
        forward[1] >= map.length ||
        forward[0] < 0 ||
        forward[0] >= map[0].length;

      const verticalMovement =
        guard.direction === "v" || guard.direction === "^";
      const currentCell = getCell(map, guard.position);

      let trail: Cell = ".";
      if (isVisited(currentCell)) {
        if (currentCell === VISITED_HORIZONTAL && verticalMovement) {
          trail = VISITED_BOTH;
        } else if (currentCell === VISITED_VERTICAL && !verticalMovement) {
          trail = VISITED_BOTH;
        } else {
          trail = currentCell;
        }
      } else if (verticalMovement) {
        trail = VISITED_VERTICAL;
      } else {
        trail = VISITED_HORIZONTAL;
      }

      if (guardWalkedOutOfMap) {
        setCell(map, guard.position, trail);
        return false; // out of map, no cycle
      }

      if (getCell(map, forward) === OBSTACLE) {
        const hit: Hit = `${forward[0]}-${forward[1]}-${guard.direction}`;
        if (hits.has(hit)) {
          return true; // cycle detected
        }

        hits.add(hit);

        setCell(map, guard.position, VISITED_BOTH);
        guard.direction = rotateRight(guard.direction);
      } else {
        setCell(map, guard.position, trail);
        guard.position = forward;
      }

      // console.log(stringifyMap(map, guard));
      // console.log("Time:", time);
    }
    time++;
  }
}

if (process.argv.includes("--part=2")) {
  putObstaclesAndSquaresAndCountCycles(parseInput(example));

  putObstaclesAndSquaresAndCountCycles(parseInput(input));
}
