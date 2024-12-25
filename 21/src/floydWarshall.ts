import type { Edge, Graph } from "./graph";

export type Path<T> = Edge<T>[];
export type ShortestPaths<T> = Map<T, Map<T, Path<T>>>;

const SAME_DIRECTION_WEIGHT = 0.9;

export function floydWarshall<T>(graph: Graph<T>): ShortestPaths<T> {
  const paths = new Map<T, Map<T, Edge<T>[]>>();
  const vertices = Array.from(graph.keys());

  for (const start of vertices) {
    let path = new Map<T, Edge<T>[]>();
    paths.set(start, path);

    for (const destination of vertices) {
      if (start === destination) {
        path.set(destination, []);
      } else {
        const edges = Array.from(graph.get(start)?.values() || []);
        const edge = edges.find((e) => e.val === destination);

        if (edge) {
          path.set(destination, [edge]);
        } else {
          path.set(destination, []);
        }
      }
    }
  }

  for (const k of vertices) {
    for (const i of vertices) {
      if (i === k) continue;

      for (const j of vertices) {
        if (j === k || j === i) continue;

        const ikPath = paths.get(i)!.get(k)!;
        const kjPath = paths.get(k)!.get(j)!;
        const currentPath = paths.get(i)!.get(j)!;

        const newPath = adjustPathWeights([...ikPath, ...kjPath]);
        if (
          ikPath.length > 0 &&
          kjPath.length > 0 &&
          (currentPath.length === 0 ||
            getPathWeight(newPath) < getPathWeight(currentPath))
        ) {
          paths.get(i)!.set(j, newPath);
        }
      }
    }
  }

  return paths;
}

function adjustPathWeights<T>(path: Path<T>): Path<T> {
  return path.map((edge, i, array) => {
    if (i === 0) return edge;

    const prev = array[i - 1];
    const curr = edge;

    if (prev.dir[0] === curr.dir[0] && prev.dir[1] === curr.dir[1]) {
      return { ...curr, weight: SAME_DIRECTION_WEIGHT };
    }

    return curr;
  });
}

function getPathWeight<T>(path: Path<T>): number {
  return path.reduce((sum, edge) => sum + edge.weight, 0);
}
