import type { Direction } from "./directions";

export type Edge<T> = {
  dir: Direction;
  val: T;
  weight: number;
};

export type Graph<T> = Map<T, Set<Edge<T>>>;
