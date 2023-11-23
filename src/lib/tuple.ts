import { TwoDimensionalArray } from './two-dimenisonal-array'

export class Tuple extends TwoDimensionalArray {
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
    super(4, 1)
    this.set(0, 0, x)
    this.set(1, 0, y)
    this.set(2, 0, z)
    this.set(3, 0, w)
  }

  get x() {
    return this.at(0)
  }

  get y() {
    return this.at(1)
  }

  get z() {
    return this.at(2)
  }

  get w() {
    return this.at(3)
  }

  get red() {
    return this.at(0)
  }

  get green() {
    return this.at(1)
  }

  get blue() {
    return this.at(2)
  }

  add(other: Tuple) {
    return new Tuple(this.x + other.x, this.y + other.y, this.z + other.z, this.w + other.w)
  }

  at(row: number, column: number = 0) {
    if (column !== 0) throw new Error(`Invalid column: ${column}`)
    return super.at(row, column)
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
    for (const value of this.values) {
      strValues.push(value.toFixed(5))
    }
    return `Tuple(${strValues.join(', ')})`
  }
}
