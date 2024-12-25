import type { Grid } from "./grid";
import type { Edge, Graph } from "./graph";
import { DIRECTIONS } from "./directions";

export function createKeypadGraph<T>(grid: Grid<T>, empty: T): Graph<T> {
  const graph: Graph<T> = new Map();
  const rows = grid.length;
  const cols = grid[0].length;

  for (let row = 0; row < rows; row++) {
    for (let col = 0; col < cols; col++) {
      if (grid[row][col] === empty) continue;

      const neighbors = new Set<Edge<T>>();

      let i = 0;
      for (const direction of DIRECTIONS) {
        const newRow = row + direction[0];
        const newCol = col + direction[1];

        if (
          newRow >= 0 &&
          newRow < rows &&
          newCol >= 0 &&
          newCol < cols &&
          grid[newRow][newCol] !== empty
        ) {
          neighbors.add({
            dir: direction,
            val: grid[newRow][newCol],
            weight: 1,
          });
        }
      }

      graph.set(grid[row][col], neighbors);
    }
  }

  return graph;
}
