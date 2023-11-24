import { Tuple } from './tuple.js'
import { TwoDimensionalArray } from './two-dimenisonal-array.js'

export class Matrix extends TwoDimensionalArray {
  static identity(size: number) {
    const matrix = new Matrix(size, size)
    for (let i = 0; i < size; i++) {
      matrix.set(i, i, 1)
    }
    return matrix
  }

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

  cofactor(row: number, column: number): number {
    return this.minor(row, column) * ((row + column) % 2 === 0 ? 1 : -1)
  }

  determinant(): number {
    if (this.rows === 2 && this.columns === 2) {
      return this.at(0, 0) * this.at(1, 1) - this.at(0, 1) * this.at(1, 0)
    }

    let determinant = 0
    for (let j = 0; j < this.columns; j++) {
      determinant += this.at(0, j) * this.cofactor(0, j)
    }
    return determinant
  }

  inverse(): Matrix {
    if (!this.isInvertible()) {
      throw new Error('Attempted to invert non-invertable matrix')
    }

    const determinant = this.determinant()

    const inverse = new Matrix(this.rows, this.columns)
    for (let i = 0; i < this.rows; i++) {
      for (let j = 0; j < this.columns; j++) {
        const cofactor = this.cofactor(i, j)
        inverse.set(j, i, cofactor / determinant)
      }
    }
    return inverse
  }

  isInvertible(): boolean {
    return this.determinant() !== 0
  }

  minor(row: number, column: number): number {
    return this.submatrix(row, column).determinant()
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

  submatrix(rowToRemove: number, columnToRemove: number): Matrix {
    const submatrix = new Matrix(this.rows - 1, this.columns - 1)
    for (let i = 0, si = 0; i < this.rows; i++) {
      if (i === rowToRemove) continue
      for (let j = 0, sj = 0; j < this.columns; j++) {
        if (j === columnToRemove) continue
        submatrix.set(si, sj, this.at(i, j))
        ++sj
      }
      ++si
    }
    return submatrix
  }

  transpose(): Matrix {
    const transposition = new Matrix(this.columns, this.rows)
    for (let i = 0; i < this.rows; i++) {
      for (let j = 0; j < this.columns; j++) {
        transposition.set(j, i, this.at(i, j))
      }
    }
    return transposition
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
