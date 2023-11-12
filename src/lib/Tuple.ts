export class Tuple {
  readonly #values: [number, number, number, 0.0 | 1.0]

  static point(x: number, y: number, z: number) {
    return new Tuple(x, y, z, 1.0)
  }

  static vector(x: number, y: number, z: number) {
    return new Tuple(x, y, z, 0.0)
  }

  constructor(x: number, y: number, z: number, w: 0.0 | 1.0) {
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

  isPoint() {
    return this.w === 1.0
  }

  isVector() {
    return this.w === 0.0
  }
}
