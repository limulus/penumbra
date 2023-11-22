export class Matrix {
  readonly rows: number
  readonly columns: number
  readonly #values: Float32Array

  constructor(values: number[][]) {
    this.rows = values[0].length
    this.columns = values.length
    this.#values = new Float32Array(this.rows * this.columns)
    for (let row = 0; row < this.rows; row++) {
      for (let col = 0; col < this.columns; col++) {
        this.#values[row * this.rows + col] = values[row][col]
      }
    }
  }

  at(row: number, column: number) {
    return this.#values[row * this.rows + column]
  }

  equals(other: Matrix) {
    if (this.rows !== other.rows || this.columns !== other.columns) {
      return false
    }
    return this.#values.every((value, index) => value === other.#values[index])
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
