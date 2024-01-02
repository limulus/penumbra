import equal from './util/equal.js'

export class TwoDimensionalArray {
  #values: Float32Array

  constructor(
    readonly rows: number,
    readonly columns: number
  ) {
    this.#values = new Float32Array(rows * columns)
  }

  protected get values() {
    return this.#values
  }

  protected set values(values: Float32Array) {
    this.#values = values
  }

  at(row: number, column: number) {
    return this.values[row * this.columns + column]
  }

  equals(other: TwoDimensionalArray) {
    if (this.rows !== other.rows || this.columns !== other.columns) {
      return false
    }
    return this.values.every((value, index) => equal(value, other.values[index]))
  }

  set(row: number, column: number, value: number) {
    this.values[row * this.columns + column] = value
  }

  toString() {
    let buffer = '\n'
    for (let row = 0; row < this.rows; row++) {
      buffer += '| '
      for (let col = 0; col < this.columns; col++) {
        buffer += `${this.at(row, col)} `
      }
      buffer += '|\n'
    }
    return buffer
  }
}
