import { Sphere } from './sphere.js'

export class Intersection {
  constructor(
    public readonly t: number,
    public readonly object: Sphere
  ) {}
}

export class IntersectionCollection {
  readonly #intersections: Intersection[] = []

  constructor(...intersections: Intersection[]) {
    this.#intersections = intersections
  }

  get length(): number {
    return this.#intersections.length
  }

  at(i: number): Intersection {
    return this.#intersections[i]
  }

  hit(): Intersection | null {
    let lowestNonnegativeIntersection: Intersection | null = null
    for (const intersection of this.#intersections) {
      if (intersection.t < 0) continue
      if (
        Math.min(lowestNonnegativeIntersection?.t ?? Infinity, intersection.t) ===
        intersection.t
      ) {
        lowestNonnegativeIntersection = intersection
      }
    }
    return lowestNonnegativeIntersection
  }
}
