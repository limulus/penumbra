import { Tuple } from './tuple.js'
import { TwoDimensionalArray } from './two-dimenisonal-array.js'

export class Matrix extends TwoDimensionalArray {
  constructor(rows: number, columns: number)
  constructor(values: number[][])
  constructor(arg1: number[][] | number, columns?: number) {
    let rows: number

    if (Array.isArray(arg1)) {
      const values = arg1
      rows = values[0].length
      columns = values.length
    } else if (typeof arg1 === 'number' && typeof columns === 'number') {
      rows = arg1
    } else {
      throw new TypeError(`Unexpected arguments to Matrix constructor: ${arg1}, ${columns}`)
    }

    super(rows, columns)

    if (Array.isArray(arg1)) {
      const values = arg1
      for (let row = 0; row < this.rows; row++) {
        for (let col = 0; col < this.columns; col++) {
          this.set(row, col, values[row][col])
        }
      }
    }
  }

  mul(other: Tuple): Tuple
  mul(other: Matrix): Matrix
  mul(other: TwoDimensionalArray): TwoDimensionalArray {
    const result =
      other instanceof Tuple ? new Tuple(0, 0, 0, 0) : new Matrix(this.rows, other.columns)

    for (let row = 0; row < this.rows; row++) {
      for (let col = 0; col < other.columns; col++) {
        let value = 0
        for (let i = 0; i < this.columns; i++) {
          value += this.at(row, i) * other.at(i, col)
        }
        result.set(row, col, value)
      }
    }

    return result
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
