export type BoardSize = { width: number; height: number };
export type Vector2 = { x: number; y: number };
export type Position = Vector2;
export type Velocity = Vector2;
export type RobotState = { position: Position; velocity: Velocity };

export function parseInput(input: string): RobotState[] {
  return input
    .trim()
    .split("\n")
    .map((line) => {
      const match = line.match(/p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)/);
      if (!match) {
        throw new Error(`Invalid input ${line}`);
      }
      const [_, px, py, vx, vy] = match.map(Number);

      return { position: { x: px, y: py }, velocity: { x: vx, y: vy } };
    });
}

function _stringifyBoard(robots: RobotState[], { width, height }: BoardSize) {
  const counts = Array.from({ length: height }, () => new Array(width).fill(0));
  const midX = Math.floor(width / 2);
  const midY = Math.floor(height / 2);

  for (const robot of robots) {
    counts[robot.position.y][robot.position.x]++;
  }

  return counts
    .map((row, y) =>
      row
        .map((count, x) => {
          const isMiddle = x === midX || y === midY;
          const char = count === 0 ? "." : count.toString();
          // we print the middle on white background
          return isMiddle ? `\x1b[47m${char}\x1b[0m` : char;
        })
        .join("")
    )
    .join("\n");
}

function solve(input: string, { width, height }: BoardSize) {
  const robots = parseInput(input);

  // simulate the robots for 100 seconds
  for (let i = 0; i < 100; i++) {
    for (const robot of robots) {
      robot.position.x = (robot.position.x + robot.velocity.x + width) % width;
      robot.position.y =
        (robot.position.y + robot.velocity.y + height) % height;
    }
  }

  // count the number of robots in each quadrant
  return getSafetyScore(robots, { width, height });
}

export function getSafetyScore(
  robots: RobotState[],
  { width, height }: BoardSize
) {
  const quadrants = {
    topLeft: 0,
    topRight: 0,
    bottomLeft: 0,
    bottomRight: 0,
  };

  const midX = Math.floor(width / 2);
  const midY = Math.floor(height / 2);

  for (const { position } of robots) {
    if (position.x < midX && position.y < midY) {
      quadrants.topLeft++;
    } else if (position.x < midX && position.y > midY) {
      quadrants.bottomLeft++;
    } else if (position.x > midX && position.y < midY) {
      quadrants.topRight++;
    } else if (position.x > midX && position.y > midY) {
      quadrants.bottomRight++;
    } else {
      // robots in the middle don't count
    }
  }

  return Object.values(quadrants).reduce((acc, val) => acc * val, 1);
}

async function main() {
  const exampleFile = Bun.file("./example.txt");
  const example = await exampleFile.text();
  console.log(solve(example, { width: 11, height: 7 }));

  const inputFile = Bun.file("./input.txt");
  const input = await inputFile.text();
  console.log(solve(input, { width: 101, height: 103 }));
}

if (import.meta.main) {
  main();
}
