import { Matrix } from './matrix.js'
import { Tuple } from './tuple.js'

export class Ray {
  constructor(
    public readonly origin: Tuple,
    public readonly direction: Tuple
  ) {}

  position(t: number): Tuple {
    return this.direction.mul(t).add(this.origin)
  }

  transform(matrix: Matrix): Ray {
    return new Ray(matrix.mul(this.origin), matrix.mul(this.direction))
  }
}
