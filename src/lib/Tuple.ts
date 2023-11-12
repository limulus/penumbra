import equal from './equal'

export class Tuple {
  readonly #values: [number, number, number, number]

  static point(x: number, y: number, z: number) {
    return new Tuple(x, y, z, 1.0)
  }

  static vector(x: number, y: number, z: number) {
    return new Tuple(x, y, z, 0.0)
  }

  constructor(x: number, y: number, z: number, w: number) {
    this.#values = [x, y, z, w]
  }

  get x() {
    return this.#values[0]
  }

  get y() {
    return this.#values[1]
  }

  get z() {
    return this.#values[2]
  }

  get w() {
    return this.#values[3]
  }

  add(other: Tuple) {
    return new Tuple(this.x + other.x, this.y + other.y, this.z + other.z, this.w + other.w)
  }

  div(divisor: number) {
    return this.mul(1 / divisor)
  }

  equals(other: Tuple) {
    return (
      equal(this.x, other.x) &&
      equal(this.y, other.y) &&
      equal(this.z, other.z) &&
      equal(this.w, other.w)
    )
  }

  isPoint(): this is { w: 1.0 } {
    return this.w === 1.0
  }

  isVector(): this is { w: 0.0 } {
    return this.w === 0.0
  }

  mul(factor: number) {
    return new Tuple(this.x * factor, this.y * factor, this.z * factor, this.w * factor)
  }

  negate() {
    return new Tuple(0, 0, 0, 0).sub(this)
  }

  sub(other: Tuple) {
    return new Tuple(this.x - other.x, this.y - other.y, this.z - other.z, this.w - other.w)
  }
}
