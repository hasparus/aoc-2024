import type { Edge, Graph } from "./graph";

export type Path<T> = Edge<T>[];
export type PathsOfEqualLength<T> = Path<T>[];
export type ShortestPaths<T> = Map<T, Map<T, PathsOfEqualLength<T>>>;

export function floydWarshall<T>(graph: Graph<T>): ShortestPaths<T> {
  const paths: ShortestPaths<T> = new Map();
  const vertices = Array.from(graph.keys());

  for (const start of vertices) {
    let path = new Map<T, PathsOfEqualLength<T>>();
    paths.set(start, path);

    for (const destination of vertices) {
      if (start === destination) {
        path.set(destination, [[]]);
      } else {
        const edges = Array.from(graph.get(start)?.values() || []);
        const edge = edges.find((e) => e.val === destination);

        if (edge) {
          path.set(destination, [[edge]]);
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

        const ikPaths = paths.get(i)!.get(k)!;
        const kjPaths = paths.get(k)!.get(j)!;

        if (ikPaths.length === 0 || kjPaths.length === 0) continue;

        const currentPaths = paths.get(i)!.get(j)!;
        const currentLength = currentPaths[0]?.length || Infinity;

        let newPaths = ikPaths.flatMap((ikPath) =>
          kjPaths.map((kjPath) => [...ikPath, ...kjPath])
        );

        let minLength = Infinity;
        for (const path of newPaths) {
          const length = path.length;
          if (length < minLength) {
            minLength = length;
          }
        }

        newPaths = newPaths.filter((p) => p.length === minLength);

        if (minLength < currentLength) {
          paths.get(i)!.set(j, newPaths);
        } else if (minLength === currentLength) {
          paths.get(i)!.set(j, [...currentPaths, ...newPaths]);
        }
      }
    }
  }

  return paths;
}
