import equal from './util/equal.js'

export class TwoDimensionalArray {
  protected values: Float32Array

  constructor(
    readonly rows: number,
    readonly columns: number
  ) {
    this.values = new Float32Array(rows * columns)
  }

  at(row: number, column: number) {
    return this.values[this.index(row, column)]
  }

  equals(other: TwoDimensionalArray) {
    if (this.rows !== other.rows || this.columns !== other.columns) {
      return false
    }
    return this.values.every((value, index) => equal(value, other.values[index]))
  }

  private index(row: number, column: number) {
    return row * this.columns + column
  }

  set(row: number, column: number, value: number) {
    this.values[this.index(row, column)] = value
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
