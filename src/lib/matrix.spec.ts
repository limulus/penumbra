import { expect } from 'chai'

import { Matrix } from './matrix.js'
import { Tuple } from './tuple.js'

describe('Matrix', () => {
  /*
  Scenario: Constructing and inspecting a 4x4 matrix
    Given the following 4x4 matrix M:
      |  1   |  2   |  3   |  4   |
      |  5.5 |  6.5 |  7.5 |  8.5 |
      |  9   | 10   | 11   | 12   |
      | 13.5 | 14.5 | 15.5 | 16.5 |
    Then M[0,0] = 1
      And M[0,3] = 4
      And M[1,0] = 5.5
      And M[1,2] = 7.5
      And M[2,2] = 11
      And M[3,0] = 13.5
      And M[3,2] = 15.5
  */
  describe('given a 4x4 matrix with unique values', () => {
    let matrix: Matrix

    beforeEach(() => {
      matrix = new Matrix([
        [1, 2, 3, 4],
        [5.5, 6.5, 7.5, 8.5],
        [9, 10, 11, 12],
        [13.5, 14.5, 15.5, 16.5],
      ])
    })

    describe('.at(0, 0)', () => {
      it('should return 1', () => {
        expect(matrix.at(0, 0)).to.approxEqual(1)
      })
    })

    describe('.at(0, 3)', () => {
      it('should return 4', () => {
        expect(matrix.at(0, 3)).to.approxEqual(4)
      })
    })

    describe('.at(1, 0)', () => {
      it('should return 5.5', () => {
        expect(matrix.at(1, 0)).to.approxEqual(5.5)
      })
    })

    describe('.at(1, 2)', () => {
      it('should return 7.5', () => {
        expect(matrix.at(1, 2)).to.approxEqual(7.5)
      })
    })

    describe('.at(2, 2)', () => {
      it('should return 11', () => {
        expect(matrix.at(2, 2)).to.approxEqual(11)
      })
    })

    describe('.at(3, 0)', () => {
      it('should return 13.5', () => {
        expect(matrix.at(3, 0)).to.approxEqual(13.5)
      })
    })

    describe('.at(3, 2)', () => {
      it('should return 15.5', () => {
        expect(matrix.at(3, 2)).to.approxEqual(15.5)
      })
    })
  })

  /*
  Scenario: A 2x2 matrix ought to be representable
    Given the following 2x2 matrix M:
      | -3 |  5 |
      |  1 | -2 |
    Then M[0,0] = -3
      And M[0,1] = 5
      And M[1,0] = 1
      And M[1,1] = -2
  */
  describe('given a 2x2 matrix with unique values', () => {
    let matrix: Matrix

    beforeEach(() => {
      matrix = new Matrix([
        [-3, 5],
        [1, -2],
      ])
    })

    describe('.at(0, 0)', () => {
      it('should return -3', () => {
        expect(matrix.at(0, 0)).to.approxEqual(-3)
      })
    })

    describe('.at(0, 1)', () => {
      it('should return 5', () => {
        expect(matrix.at(0, 1)).to.approxEqual(5)
      })
    })

    describe('.at(1, 0)', () => {
      it('should return 1', () => {
        expect(matrix.at(1, 0)).to.approxEqual(1)
      })
    })

    describe('.at(1, 1)', () => {
      it('should return -2', () => {
        expect(matrix.at(1, 1)).to.approxEqual(-2)
      })
    })
  })

  /*
  Scenario: A 3x3 matrix ought to be representable
    Given the following 3x3 matrix M:
      | -3 |  5 |  0 |
      |  1 | -2 | -7 |
      |  0 |  1 |  1 |
    Then M[0,0] = -3
      And M[1,1] = -2
      And M[2,2] = 1
  */
  describe('given a 3x3 matrix with unique values', () => {
    let matrix: Matrix

    beforeEach(() => {
      matrix = new Matrix([
        [-3, 5, 0],
        [1, -2, -7],
        [0, 1, 1],
      ])
    })

    describe('.at(0, 0)', () => {
      it('should return -3', () => {
        expect(matrix.at(0, 0)).to.approxEqual(-3)
      })
    })

    describe('.at(1, 1)', () => {
      it('should return -2', () => {
        expect(matrix.at(1, 1)).to.approxEqual(-2)
      })
    })

    describe('.at(2, 2)', () => {
      it('should return 1', () => {
        expect(matrix.at(2, 2)).to.approxEqual(1)
      })
    })
  })

  /*
  Scenario: Matrix equality with identical matrices
    Given the following matrix A:
        | 1 | 2 | 3 | 4 |
        | 5 | 6 | 7 | 8 |
        | 9 | 8 | 7 | 6 |
        | 5 | 4 | 3 | 2 |
      And the following matrix B:
        | 1 | 2 | 3 | 4 |
        | 5 | 6 | 7 | 8 |
        | 9 | 8 | 7 | 6 |
        | 5 | 4 | 3 | 2 |
    Then A = B
  */
  describe('given two identical matrices', () => {
    let a: Matrix
    let b: Matrix

    beforeEach(() => {
      a = new Matrix([
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 8, 7, 6],
        [5, 4, 3, 2],
      ])
      b = new Matrix([
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 8, 7, 6],
        [5, 4, 3, 2],
      ])
    })

    describe('when comparing A to B', () => {
      it('should return true', () => {
        expect(a).to.equal(b)
      })
    })
  })

  /*
  Scenario: Matrix equality with different matrices
    Given the following matrix A:
        | 1 | 2 | 3 | 4 |
        | 5 | 6 | 7 | 8 |
        | 9 | 8 | 7 | 6 |
        | 5 | 4 | 3 | 2 |
      And the following matrix B:
        | 2 | 3 | 4 | 5 |
        | 6 | 7 | 8 | 9 |
        | 8 | 7 | 6 | 5 |
        | 4 | 3 | 2 | 1 |
    Then A != B
  */
  describe('given two different matrices', () => {
    let a: Matrix
    let b: Matrix

    beforeEach(() => {
      a = new Matrix([
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 8, 7, 6],
        [5, 4, 3, 2],
      ])
      b = new Matrix([
        [2, 3, 4, 5],
        [6, 7, 8, 9],
        [8, 7, 6, 5],
        [4, 3, 2, 1],
      ])
    })

    describe('when comparing A to B', () => {
      it('should return false', () => {
        expect(a).to.not.equal(b)
      })
    })
  })

  /*
  Scenario: Multiplying two matrices
    Given the following matrix A:
        | 1 | 2 | 3 | 4 |
        | 5 | 6 | 7 | 8 |
        | 9 | 8 | 7 | 6 |
        | 5 | 4 | 3 | 2 |
      And the following matrix B:
        | -2 | 1 | 2 |  3 |
        |  3 | 2 | 1 | -1 |
        |  4 | 3 | 6 |  5 |
        |  1 | 2 | 7 |  8 |
    Then A * B is the following 4x4 matrix:
        | 20|  22 |  50 |  48 |
        | 44|  54 | 114 | 108 |
        | 40|  58 | 110 | 102 |
        | 16|  26 |  46 |  42 |
  */
  describe('given two matrices', () => {
    let a: Matrix
    let b: Matrix

    beforeEach(() => {
      a = new Matrix([
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 8, 7, 6],
        [5, 4, 3, 2],
      ])
      b = new Matrix([
        [-2, 1, 2, 3],
        [3, 2, 1, -1],
        [4, 3, 6, 5],
        [1, 2, 7, 8],
      ])
    })

    describe('when multiplying A by B', () => {
      it('should return the expected matrix', () => {
        expect(a.mul(b)).to.equal(
          new Matrix([
            [20, 22, 50, 48],
            [44, 54, 114, 108],
            [40, 58, 110, 102],
            [16, 26, 46, 42],
          ])
        )
      })
    })
  })

  /*
  Scenario: A matrix multiplied by a tuple
    Given the following matrix A:
        | 1 | 2 | 3 | 4 |
        | 2 | 4 | 4 | 2 |
        | 8 | 6 | 4 | 1 |
        | 0 | 0 | 0 | 1 |
      And b ← tuple(1, 2, 3, 1)
    Then A * b = tuple(18, 24, 33, 1)
  */
  describe('given a matrix and a tuple', () => {
    let A: Matrix
    let b: Tuple

    beforeEach(() => {
      A = new Matrix([
        [1, 2, 3, 4],
        [2, 4, 4, 2],
        [8, 6, 4, 1],
        [0, 0, 0, 1],
      ])
      b = new Tuple(1, 2, 3, 1)
      console.log(b.toString())
    })

    describe('when multiplying A by b', () => {
      it('should return the expected tuple', () => {
        expect(A.mul(b)).to.equal(new Tuple(18, 24, 33, 1))
      })
    })
  })
})
