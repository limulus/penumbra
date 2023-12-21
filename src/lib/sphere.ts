import { Intersection, IntersectionCollection } from './intersection.js'
import { Matrix } from './matrix.js'
import { Ray } from './ray.js'
import { Tuple } from './tuple.js'

const identityMatrix = Matrix.identity(4)
const origin = Tuple.point(0, 0, 0)

export class Sphere {
  #transform: Matrix = identityMatrix
  get transform(): Matrix {
    return this.#transform
  }
  set transform(transform: Matrix) {
    this.#transformInverseCache = undefined
    this.#transform = transform
  }

  #transformInverseCache?: Matrix
  private get transformInverse(): Matrix {
    if (!this.#transformInverseCache) {
      this.#transformInverseCache = this.#transform.inverse()
    }
    return this.#transformInverseCache
  }

  intersect(ray: Ray): IntersectionCollection {
    // Transform the ray into object space
    ray = ray.transform(this.transformInverse)

    // Vector from sphere origin to the ray origin
    const sphereToRayVec = ray.origin.sub(origin)

    // Supporting characters to determine the discriminant and intersection
    const a = ray.direction.dot(ray.direction)
    const b = 2 * ray.direction.dot(sphereToRayVec)
    const c = sphereToRayVec.dot(sphereToRayVec) - 1

    // Discriminant does not intersect sphere if it is negative
    const discriminant = b ** 2 - 4 * a * c
    if (discriminant < 0) return new IntersectionCollection()

    // Calculate the intersection points
    const sqrtDiscriminant = Math.sqrt(discriminant)
    const t1 = (-b - sqrtDiscriminant) / (2 * a)
    const t2 = (-b + sqrtDiscriminant) / (2 * a)
    return new IntersectionCollection(
      new Intersection(t1, this),
      new Intersection(t2, this)
    )
  }
}
