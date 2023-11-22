import equal from './util/equal'

export class Tuple {
  readonly #values: Float32Array

  static color(red: number, green: number, blue: number) {
    return new Tuple(red, green, blue, 0.0)
  }

  static point(x: number, y: number, z: number) {
    return new Tuple(x, y, z, 1.0)
  }

  static vector(x: number, y: number, z: number) {
    return new Tuple(x, y, z, 0.0)
  }

  constructor(x: number, y: number, z: number, w: number) {
    this.#values = new Float32Array([x, y, z, w])
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

  get red() {
    return this.#values[0]
  }

  get green() {
    return this.#values[1]
  }

  get blue() {
    return this.#values[2]
  }

  add(other: Tuple) {
    return new Tuple(this.x + other.x, this.y + other.y, this.z + other.z, this.w + other.w)
  }

  cross(other: Tuple) {
    return Tuple.vector(
      this.y * other.z - this.z * other.y,
      this.z * other.x - this.x * other.z,
      this.x * other.y - this.y * other.x
    )
  }

  div(divisor: number) {
    return this.mul(1 / divisor)
  }

  dot(other: Tuple) {
    return this.x * other.x + this.y * other.y + this.z * other.z + this.w * other.w
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

  magnitude() {
    return Math.sqrt(this.x ** 2 + this.y ** 2 + this.z ** 2 + this.w ** 2)
  }

  mul(factor: number | Tuple): Tuple {
    if (typeof factor === 'number') {
      return new Tuple(this.x * factor, this.y * factor, this.z * factor, this.w * factor)
    } else if (factor instanceof Tuple) {
      return new Tuple(
        this.x * factor.x,
        this.y * factor.y,
        this.z * factor.z,
        this.w * factor.w
      )
    } else {
      throw new Error(`Invalid factor: ${factor}`)
    }
  }

  negate() {
    return new Tuple(0, 0, 0, 0).sub(this)
  }

  normalize() {
    return this.div(this.magnitude())
  }

  sub(other: Tuple) {
    return new Tuple(this.x - other.x, this.y - other.y, this.z - other.z, this.w - other.w)
  }

  toString() {
    const strValues: string[] = []
    for (const value of this.#values) {
      strValues.push(value.toFixed(5))
    }
    return `Tuple(${strValues.join(', ')})`
  }
}
