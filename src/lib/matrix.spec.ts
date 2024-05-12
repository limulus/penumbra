import '../test/setup.js'

import { expect } from 'chai'
import { describe, beforeEach, it } from 'vitest'

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
    })

    describe('when multiplying A by b', () => {
      it('should return the expected tuple', () => {
        expect(A.mul(b)).to.equal(new Tuple(18, 24, 33, 1))
      })
    })
  })

  /*
  Scenario: Multiplying a matrix by the identity matrix
    Given the following matrix A:
      | 0 | 1 |  2 |  4 |
      | 1 | 2 |  4 |  8 |
      | 2 | 4 |  8 | 16 |
      | 4 | 8 | 16 | 32 |
    Then A * identity_matrix = A
  */
  describe('given a matrix and the identity matrix', () => {
    let A: Matrix
    let identity: Matrix

    beforeEach(() => {
      A = new Matrix([
        [0, 1, 2, 4],
        [1, 2, 4, 8],
        [2, 4, 8, 16],
        [4, 8, 16, 32],
      ])
      identity = Matrix.identity(4)
    })

    describe('when multiplying A by the identity matrix', () => {
      it('should return A', () => {
        expect(A.mul(identity)).to.equal(A)
      })
    })
  })

  /*
  Scenario: Multiplying the identity matrix by a tuple
    Given a ← tuple(1, 2, 3, 4)
    Then identity_matrix * a = a
  */
  describe('given a tuple and the identity matrix', () => {
    let a: Tuple
    let identity: Matrix

    beforeEach(() => {
      a = new Tuple(1, 2, 3, 4)
      identity = Matrix.identity(4)
    })

    describe('when multiplying the identity matrix by a', () => {
      it('should return a', () => {
        expect(identity.mul(a)).to.equal(a)
      })
    })
  })

  /*
  Scenario: Transposing a matrix
    Given the following matrix A:
      | 0 | 9 | 3 | 0 |
      | 9 | 8 | 0 | 8 |
      | 1 | 8 | 5 | 3 |
      | 0 | 0 | 5 | 8 |
    Then transpose(A) is the following matrix:
      | 0 | 9 | 1 | 0 |
      | 9 | 8 | 8 | 0 |
      | 3 | 0 | 5 | 5 |
      | 0 | 8 | 3 | 8 |
  */
  describe('given a matrix', () => {
    let A: Matrix

    beforeEach(() => {
      A = new Matrix([
        [0, 9, 3, 0],
        [9, 8, 0, 8],
        [1, 8, 5, 3],
        [0, 0, 5, 8],
      ])
    })

    describe('when transposing A', () => {
      it('should return the expected matrix', () => {
        expect(A.transpose()).to.equal(
          new Matrix([
            [0, 9, 1, 0],
            [9, 8, 8, 0],
            [3, 0, 5, 5],
            [0, 8, 3, 8],
          ])
        )
      })
    })
  })

  /*
  Scenario: Transposing the identity matrix
    Given A ← transpose(identity_matrix)
    Then A = identity_matrix
  */
  describe('given the identity matrix', () => {
    let identity: Matrix
    let A: Matrix

    beforeEach(() => {
      identity = Matrix.identity(4)
      A = identity.transpose()
    })

    describe('when transposing A', () => {
      it('should return the identity matrix', () => {
        expect(A).to.equal(identity)
      })
    })
  })

  /*
  Scenario: Calculating the determinant of a 2x2 matrix
  Given the following 2x2 matrix A:
    |  1 | 5 |
    | -3 | 2 |
  Then determinant(A) = 17
  */
  describe('given a 2x2 matrix', () => {
    let A: Matrix

    beforeEach(() => {
      A = new Matrix([
        [1, 5],
        [-3, 2],
      ])
    })

    describe('when calculating the determinant of A', () => {
      it('should return 17', () => {
        expect(A.determinant()).to.equal(17)
      })
    })
  })

  /*
  Scenario: A submatrix of a 3x3 matrix is a 2x2 matrix
    Given the following 3x3 matrix A:
      |  1 | 5 |  0 |
      | -3 | 2 |  7 |
      |  0 | 6 | -3 |
    Then submatrix(A, 0, 2) is the following 2x2 matrix:
      | -3 | 2 |
      |  0 | 6 |
  */
  describe('given a 3x3 matrix', () => {
    let A: Matrix

    beforeEach(() => {
      A = new Matrix([
        [1, 5, 0],
        [-3, 2, 7],
        [0, 6, -3],
      ])
    })

    describe('when calculating the submatrix of A', () => {
      it('should return the expected matrix', () => {
        expect(A.submatrix(0, 2)).to.equal(
          new Matrix([
            [-3, 2],
            [0, 6],
          ])
        )
      })
    })
  })

  /*
  Scenario: A submatrix of a 4x4 matrix is a 3x3 matrix
    Given the following 4x4 matrix A:
      | -6 |  1 |  1 |  6 |
      | -8 |  5 |  8 |  6 |
      | -1 |  0 |  8 |  2 |
      | -7 |  1 | -1 |  1 |
    Then submatrix(A, 2, 1) is the following 3x3 matrix:
      | -6 |  1 | 6 |
      | -8 |  8 | 6 |
      | -7 | -1 | 1 |
  */
  describe('given a 4x4 matrix', () => {
    let A: Matrix

    beforeEach(() => {
      A = new Matrix([
        [-6, 1, 1, 6],
        [-8, 5, 8, 6],
        [-1, 0, 8, 2],
        [-7, 1, -1, 1],
      ])
    })

    describe('when calculating the submatrix of A', () => {
      it('should return the expected matrix', () => {
        expect(A.submatrix(2, 1)).to.equal(
          new Matrix([
            [-6, 1, 6],
            [-8, 8, 6],
            [-7, -1, 1],
          ])
        )
      })
    })
  })

  /*
  Scenario: Calculating a minor of a 3x3 matrix
    Given the following 3x3 matrix A:
        |  3 |  5 |  0 |
        |  2 | -1 | -7 |
        |  6 | -1 |  5 |
      And B ← submatrix(A, 1, 0)
    Then determinant(B) = 25
      And minor(A, 1, 0) = 25
  */
  describe('given a 3x3 matrix', () => {
    let A: Matrix
    let B: Matrix

    beforeEach(() => {
      A = new Matrix([
        [3, 5, 0],
        [2, -1, -7],
        [6, -1, 5],
      ])
      B = A.submatrix(1, 0)
    })

    describe('when calculating the determinant of B', () => {
      it('should return 25', () => {
        expect(B.determinant()).to.equal(25)
      })
    })

    describe('when calculating the minor of A', () => {
      it('should return 25', () => {
        expect(A.minor(1, 0)).to.equal(25)
      })
    })
  })

  /*
  Scenario: Calculating a cofactor of a 3x3 matrix
    Given the following 3x3 matrix A:
        |  3 |  5 |  0 |
        |  2 | -1 | -7 |
        |  6 | -1 |  5 |
    Then minor(A, 0, 0) = -12
      And cofactor(A, 0, 0) = -12
      And minor(A, 1, 0) = 25
      And cofactor(A, 1, 0) = -25
  */
  describe('given a 3x3 matrix', () => {
    let A: Matrix

    beforeEach(() => {
      A = new Matrix([
        [3, 5, 0],
        [2, -1, -7],
        [6, -1, 5],
      ])
    })

    describe('when calculating the minor of A at 0,0', () => {
      it('should return -12', () => {
        expect(A.minor(0, 0)).to.equal(-12)
      })
    })

    describe('when calculating the cofactor of A at 0,0', () => {
      it('should return -12', () => {
        expect(A.cofactor(0, 0)).to.equal(-12)
      })
    })

    describe('when calculating the minor of A at 1,0', () => {
      it('should return 25', () => {
        expect(A.minor(1, 0)).to.equal(25)
      })
    })

    describe('when calculating the cofactor of A at 1,0', () => {
      it('should return -25', () => {
        expect(A.cofactor(1, 0)).to.equal(-25)
      })
    })
  })

  /*
  Scenario: Calculating the determinant of a 3x3 matrix
    Given the following 3x3 matrix A:
      |  1 |  2 |  6 |
      | -5 |  8 | -4 |
      |  2 |  6 |  4 |
    Then cofactor(A, 0, 0) = 56
      And cofactor(A, 0, 1) = 12
      And cofactor(A, 0, 2) = -46
      And determinant(A) = -196
  */
  describe('given a 3x3 matrix', () => {
    let A: Matrix

    beforeEach(() => {
      A = new Matrix([
        [1, 2, 6],
        [-5, 8, -4],
        [2, 6, 4],
      ])
    })

    describe('when calculating the cofactor of A at 0,0', () => {
      it('should return 56', () => {
        expect(A.cofactor(0, 0)).to.equal(56)
      })
    })

    describe('when calculating the cofactor of A at 0,1', () => {
      it('should return 12', () => {
        expect(A.cofactor(0, 1)).to.equal(12)
      })
    })

    describe('when calculating the cofactor of A at 0,2', () => {
      it('should return -46', () => {
        expect(A.cofactor(0, 2)).to.equal(-46)
      })
    })

    describe('when calculating the determinant of A', () => {
      it('should return -196', () => {
        expect(A.determinant()).to.equal(-196)
      })
    })
  })

  /*
  Scenario: Calculating the determinant of a 4x4 matrix
    Given the following 4x4 matrix A:
      | -2 | -8 |  3 |  5 |
      | -3 |  1 |  7 |  3 |
      |  1 |  2 | -9 |  6 |
      | -6 |  7 |  7 | -9 |
    Then cofactor(A, 0, 0) = 690
      And cofactor(A, 0, 1) = 447
      And cofactor(A, 0, 2) = 210
      And cofactor(A, 0, 3) = 51
      And determinant(A) = -4071
  */
  describe('given a 4x4 matrix', () => {
    let A: Matrix

    beforeEach(() => {
      A = new Matrix([
        [-2, -8, 3, 5],
        [-3, 1, 7, 3],
        [1, 2, -9, 6],
        [-6, 7, 7, -9],
      ])
    })

    describe('when calculating the cofactor of A at 0,0', () => {
      it('should return 690', () => {
        expect(A.cofactor(0, 0)).to.equal(690)
      })
    })

    describe('when calculating the cofactor of A at 0,1', () => {
      it('should return 447', () => {
        expect(A.cofactor(0, 1)).to.equal(447)
      })
    })

    describe('when calculating the cofactor of A at 0,2', () => {
      it('should return 210', () => {
        expect(A.cofactor(0, 2)).to.equal(210)
      })
    })

    describe('when calculating the cofactor of A at 0,3', () => {
      it('should return 51', () => {
        expect(A.cofactor(0, 3)).to.equal(51)
      })
    })

    describe('when calculating the determinant of A', () => {
      it('should return -4071', () => {
        expect(A.determinant()).to.equal(-4071)
      })
    })
  })

  /*
  Scenario: Testing an invertible matrix for invertibility
    Given the following 4x4 matrix A:
      |  6 |  4 |  4 |  4 |
      |  5 |  5 |  7 |  6 |
      |  4 | -9 |  3 | -7 |
      |  9 |  1 |  7 | -6 |
    Then determinant(A) = -2120
      And A is invertible
  */
  describe('given a 4x4 matrix', () => {
    let A: Matrix

    beforeEach(() => {
      A = new Matrix([
        [6, 4, 4, 4],
        [5, 5, 7, 6],
        [4, -9, 3, -7],
        [9, 1, 7, -6],
      ])
    })

    describe('when calculating the determinant of A', () => {
      it('should return -2120', () => {
        expect(A.determinant()).to.equal(-2120)
      })
    })

    describe('when checking if A is invertible', () => {
      it('should return true', () => {
        expect(A.isInvertible()).to.be.true
      })
    })
  })

  /*
  Scenario: Testing a noninvertible matrix for invertibility
    Given the following 4x4 matrix A:
      | -4 |  2 | -2 | -3 |
      |  9 |  6 |  2 |  6 |
      |  0 | -5 |  1 | -5 |
      |  0 |  0 |  0 |  0 |
    Then determinant(A) = 0
      And A is not invertible
  */
  describe('given a 4x4 matrix', () => {
    let A: Matrix

    beforeEach(() => {
      A = new Matrix([
        [-4, 2, -2, -3],
        [9, 6, 2, 6],
        [0, -5, 1, -5],
        [0, 0, 0, 0],
      ])
    })

    describe('when calculating the determinant of A', () => {
      it('should return 0', () => {
        expect(A.determinant()).to.equal(0)
      })
    })

    describe('when checking if A is invertible', () => {
      it('should return false', () => {
        expect(A.isInvertible()).to.be.false
      })
    })
  })

  /*
  Scenario: Calculating the inverse of a matrix
    Given the following 4x4 matrix A:
        | -5 |  2 |  6 | -8 |
        |  1 | -5 |  1 |  8 |
        |  7 |  7 | -6 | -7 |
        |  1 | -3 |  7 |  4 |
      And B ← inverse(A)
    Then determinant(A) = 532
      And cofactor(A, 2, 3) = -160
      And B[3,2] = -160/532
      And cofactor(A, 3, 2) = 105
      And B[2,3] = 105/532
      And B is the following 4x4 matrix:
        |  0.21805 |  0.45113 |  0.24060 | -0.04511 |
        | -0.80827 | -1.45677 | -0.44361 |  0.52068 |
        | -0.07895 | -0.22368 | -0.05263 |  0.19737 |
        | -0.52256 | -0.81391 | -0.30075 |  0.30639 |
  */
  describe('given a 4x4 matrix', () => {
    let A: Matrix
    let B: Matrix

    beforeEach(() => {
      A = new Matrix([
        [-5, 2, 6, -8],
        [1, -5, 1, 8],
        [7, 7, -6, -7],
        [1, -3, 7, 4],
      ])
      B = A.inverse()
    })

    describe('when calculating the determinant of A', () => {
      it('should return 532', () => {
        expect(A.determinant()).to.equal(532)
      })
    })

    describe('when calculating the cofactor of A at 2,3', () => {
      it('should return -160', () => {
        expect(A.cofactor(2, 3)).to.equal(-160)
      })
    })

    describe('when checking B[3,2]', () => {
      it('should return -160/532', () => {
        expect(B.at(3, 2)).to.approxEqual(-160 / 532)
      })
    })

    describe('when calculating the cofactor of A at 3,2', () => {
      it('should return 105', () => {
        expect(A.cofactor(3, 2)).to.equal(105)
      })
    })

    describe('when checking B[2,3]', () => {
      it('should return 105/532', () => {
        expect(B.at(2, 3)).to.approxEqual(105 / 532)
      })
    })

    describe('when checking B', () => {
      it('should return the expected matrix', () => {
        expect(B).to.equal(
          new Matrix([
            [0.21805, 0.45113, 0.2406, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
          ])
        )
      })
    })
  })

  /*
  Scenario: Calculating the inverse of another matrix
    Given the following 4x4 matrix A:
      |  8 | -5 |  9 |  2 |
      |  7 |  5 |  6 |  1 |
      | -6 |  0 |  9 |  6 |
      | -3 |  0 | -9 | -4 |
    Then inverse(A) is the following 4x4 matrix:
      | -0.15385 | -0.15385 | -0.28205 | -0.53846 |
      | -0.07692 |  0.12308 |  0.02564 |  0.03077 |
      |  0.35897 |  0.35897 |  0.43590 |  0.92308 |
      | -0.69231 | -0.69231 | -0.76923 | -1.92308 |
  */
  describe('given a 4x4 matrix', () => {
    let A: Matrix

    beforeEach(() => {
      A = new Matrix([
        [8, -5, 9, 2],
        [7, 5, 6, 1],
        [-6, 0, 9, 6],
        [-3, 0, -9, -4],
      ])
    })

    describe('when calculating the inverse of A', () => {
      it('should return the expected matrix', () => {
        expect(A.inverse()).to.equal(
          new Matrix([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.4359, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
          ])
        )
      })
    })
  })

  /*
  Scenario: Calculating the inverse of a third matrix
    Given the following 4x4 matrix A:
      |  9 |  3 |  0 |  9 |
      | -5 | -2 | -6 | -3 |
      | -4 |  9 |  6 |  4 |
      | -7 |  6 |  6 |  2 |
    Then inverse(A) is the following 4x4 matrix:
      | -0.04074 | -0.07778 |  0.14444 | -0.22222 |
      | -0.07778 |  0.03333 |  0.36667 | -0.33333 |
      | -0.02901 | -0.14630 | -0.10926 |  0.12963 |
      |  0.17778 |  0.06667 | -0.26667 |  0.33333 |
  */
  describe('given a 4x4 matrix', () => {
    let A: Matrix

    beforeEach(() => {
      A = new Matrix([
        [9, 3, 0, 9],
        [-5, -2, -6, -3],
        [-4, 9, 6, 4],
        [-7, 6, 6, 2],
      ])
    })

    describe('when calculating the inverse of A', () => {
      it('should return the expected matrix', () => {
        expect(A.inverse()).to.equal(
          new Matrix([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.1463, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
          ])
        )
      })
    })
  })

  /*
  Scenario: Multiplying a product by its inverse
    Given the following 4x4 matrix A:
        |  3 | -9 |  7 |  3 |
        |  3 | -8 |  2 | -9 |
        | -4 |  4 |  4 |  1 |
        | -6 |  5 | -1 |  1 |
      And the following 4x4 matrix B:
        |  8 |  2 |  2 |  2 |
        |  3 | -1 |  7 |  0 |
        |  7 |  0 |  5 |  4 |
        |  6 | -2 |  0 |  5 |
      And C ← A * B
    Then C * inverse(B) = A
  */
  describe('given two 4x4 matrices', () => {
    let A: Matrix
    let B: Matrix
    let C: Matrix

    beforeEach(() => {
      A = new Matrix([
        [3, -9, 7, 3],
        [3, -8, 2, -9],
        [-4, 4, 4, 1],
        [-6, 5, -1, 1],
      ])
      B = new Matrix([
        [8, 2, 2, 2],
        [3, -1, 7, 0],
        [7, 0, 5, 4],
        [6, -2, 0, 5],
      ])
      C = A.mul(B)
    })

    describe('when multiplying C by the inverse of B', () => {
      it('should return A', () => {
        expect(C.mul(B.inverse())).to.equal(A)
      })
    })
  })

  describe('given the idenity matrix', () => {
    let identity: Matrix

    beforeEach(() => {
      identity = Matrix.identity(4)
    })

    describe('when calculating the inverse of the identity matrix', () => {
      it('should return the identity matrix', () => {
        expect(identity.inverse()).to.equal(identity)
      })
    })
  })

  describe('given a matrix and its inverse', () => {
    let A: Matrix
    let B: Matrix

    beforeEach(() => {
      A = new Matrix([
        [3, -9, 7, 3],
        [3, -8, 2, -9],
        [-4, 4, 4, 1],
        [-6, 5, -1, 1],
      ])
      B = A.inverse()
    })

    describe('when multiplying A by B', () => {
      it('should return the identity matrix', () => {
        expect(A.mul(B)).to.equal(Matrix.identity(4))
      })
    })
  })

  describe('given a matrix A and its inverse of its transpose B', () => {
    let A: Matrix
    let B: Matrix

    beforeEach(() => {
      A = new Matrix([
        [3, -9, 7, 3],
        [3, -8, 2, -9],
        [-4, 4, 4, 1],
        [-6, 5, -1, 1],
      ])
      B = A.inverse().transpose()
    })

    it('should equal the transpose of its inverse', () => {
      expect(A.transpose().inverse()).to.equal(B)
    })
  })

  /*
    Scenario: Multiplying by a translation matrix
      Given transform ← translation(5, -3, 2)
        And p ← point(-3, 4, 5)
      Then transform * p = point(2, 1, 7)
  */
  describe('given a translation matrix and a point', () => {
    let transform: Matrix
    let p: Tuple

    beforeEach(() => {
      transform = Matrix.translation(5, -3, 2)
      p = Tuple.point(-3, 4, 5)
    })

    describe('when multiplying transform by p', () => {
      it('should return the expected point', () => {
        expect(transform.mul(p)).to.equal(Tuple.point(2, 1, 7))
      })
    })
  })

  /*
    Scenario: Multiplying by the inverse of a translation matrix
      Given transform ← translation(5, -3, 2)
        And inv ← inverse(transform)
        And p ← point(-3, 4, 5)
      Then inv * p = point(-8, 7, 3)
  */
  describe('given a translation matrix and a point', () => {
    let transform: Matrix
    let inv: Matrix
    let p: Tuple

    beforeEach(() => {
      transform = Matrix.translation(5, -3, 2)
      inv = transform.inverse()
      p = Tuple.point(-3, 4, 5)
    })

    describe('when multiplying inv by p', () => {
      it('should return the expected point', () => {
        expect(inv.mul(p)).to.equal(Tuple.point(-8, 7, 3))
      })
    })
  })

  /*
    Scenario: Translation does not affect vectors
      Given transform ← translation(5, -3, 2)
        And v ← vector(-3, 4, 5)
      Then transform * v = v
  */
  describe('given a translation matrix and a vector', () => {
    let transform: Matrix
    let v: Tuple

    beforeEach(() => {
      transform = Matrix.translation(5, -3, 2)
      v = Tuple.vector(-3, 4, 5)
    })

    describe('when multiplying transform by v', () => {
      it('should return v', () => {
        expect(transform.mul(v)).to.equal(v)
      })
    })
  })

  /*
    Scenario: A scaling matrix applied to a point
      Given transform ← scaling(2, 3, 4)
        And p ← point(-4, 6, 8)
      Then transform * p = point(-8, 18, 32)
  */
  describe('given a scaling matrix and a point', () => {
    let transform: Matrix
    let p: Tuple

    beforeEach(() => {
      transform = Matrix.scaling(2, 3, 4)
      p = Tuple.point(-4, 6, 8)
    })

    describe('when multiplying transform by p', () => {
      it('should return the expected point', () => {
        expect(transform.mul(p)).to.equal(Tuple.point(-8, 18, 32))
      })
    })
  })

  /*
    Scenario: A scaling matrix applied to a vector
      Given transform ← scaling(2, 3, 4)
        And v ← vector(-4, 6, 8)
      Then transform * v = vector(-8, 18, 32)
  */
  describe('given a scaling matrix and a vector', () => {
    let transform: Matrix
    let v: Tuple

    beforeEach(() => {
      transform = Matrix.scaling(2, 3, 4)
      v = Tuple.vector(-4, 6, 8)
    })

    describe('when multiplying transform by v', () => {
      it('should return the expected vector', () => {
        expect(transform.mul(v)).to.equal(Tuple.vector(-8, 18, 32))
      })
    })
  })

  /*
  Scenario: Multiplying by the inverse of a scaling matrix
    Given transform ← scaling(2, 3, 4)
      And inv ← inverse(transform)
      And v ← vector(-4, 6, 8)
    Then inv * v = vector(-2, 2, 2)
  */
  describe('given a scaling matrix and a vector', () => {
    let transform: Matrix
    let inv: Matrix
    let v: Tuple

    beforeEach(() => {
      transform = Matrix.scaling(2, 3, 4)
      inv = transform.inverse()
      v = Tuple.vector(-4, 6, 8)
    })

    describe('when multiplying inv by v', () => {
      it('should return the expected vector', () => {
        expect(inv.mul(v)).to.equal(Tuple.vector(-2, 2, 2))
      })
    })
  })

  /*
    Scenario: Reflection is scaling by a negative value
      Given transform ← scaling(-1, 1, 1)
        And p ← point(2, 3, 4)
      Then transform * p = point(-2, 3, 4)
  */
  describe('given a scaling matrix and a point', () => {
    let transform: Matrix
    let p: Tuple

    beforeEach(() => {
      transform = Matrix.scaling(-1, 1, 1)
      p = Tuple.point(2, 3, 4)
    })

    describe('when multiplying transform by p', () => {
      it('should return the expected point', () => {
        expect(transform.mul(p)).to.equal(Tuple.point(-2, 3, 4))
      })
    })
  })

  /*
    Scenario: Rotating a point around the x axis
      Given p ← point(0, 1, 0)
        And half_quarter ← rotation_x(π / 4)
        And full_quarter ← rotation_x(π / 2)
      Then half_quarter * p = point(0, √2/2, √2/2)
        And full_quarter * p = point(0, 0, 1)
  */
  describe('given a point and two x rotation matrices', () => {
    let p: Tuple
    let halfQuarter: Matrix
    let fullQuarter: Matrix

    beforeEach(() => {
      p = Tuple.point(0, 1, 0)
      halfQuarter = Matrix.rotationX(Math.PI / 4)
      fullQuarter = Matrix.rotationX(Math.PI / 2)
    })

    describe('when multiplying halfQuarter by p', () => {
      it('should return the expected point', () => {
        expect(halfQuarter.mul(p)).to.equal(
          Tuple.point(0, Math.sqrt(2) / 2, Math.sqrt(2) / 2)
        )
      })
    })

    describe('when multiplying fullQuarter by p', () => {
      it('should return the expected point', () => {
        expect(fullQuarter.mul(p)).to.equal(Tuple.point(0, 0, 1))
      })
    })
  })

  /*
    Scenario: The inverse of an x-rotation rotates in the opposite direction
      Given p ← point(0, 1, 0)
        And half_quarter ← rotation_x(π / 4)
        And inv ← inverse(half_quarter)
      Then inv * p = point(0, √2/2, -√2/2)
  */
  describe('given a point and an x rotation matrix', () => {
    let p: Tuple
    let halfQuarter: Matrix
    let inv: Matrix

    beforeEach(() => {
      p = Tuple.point(0, 1, 0)
      halfQuarter = Matrix.rotationX(Math.PI / 4)
      inv = halfQuarter.inverse()
    })

    describe('when multiplying inv by p', () => {
      it('should return the expected point', () => {
        expect(inv.mul(p)).to.equal(Tuple.point(0, Math.sqrt(2) / 2, -Math.sqrt(2) / 2))
      })
    })
  })

  /*
    Scenario: Rotating a point around the y axis
      Given p ← point(0, 0, 1)
        And half_quarter ← rotation_y(π / 4)
        And full_quarter ← rotation_y(π / 2)
      Then half_quarter * p = point(√2/2, 0, √2/2)
        And full_quarter * p = point(1, 0, 0)
  */
  describe('given a point and two y rotation matrices', () => {
    let p: Tuple
    let halfQuarter: Matrix
    let fullQuarter: Matrix

    beforeEach(() => {
      p = Tuple.point(0, 0, 1)
      halfQuarter = Matrix.rotationY(Math.PI / 4)
      fullQuarter = Matrix.rotationY(Math.PI / 2)
    })

    describe('when multiplying halfQuarter by p', () => {
      it('should return the expected point', () => {
        expect(halfQuarter.mul(p)).to.equal(
          Tuple.point(Math.sqrt(2) / 2, 0, Math.sqrt(2) / 2)
        )
      })
    })

    describe('when multiplying fullQuarter by p', () => {
      it('should return the expected point', () => {
        expect(fullQuarter.mul(p)).to.equal(Tuple.point(1, 0, 0))
      })
    })
  })

  /*
  Scenario: Rotating a point around the z axis
    Given p ← point(0, 1, 0)
      And half_quarter ← rotation_z(π / 4)
      And full_quarter ← rotation_z(π / 2)
    Then half_quarter * p = point(-√2/2, √2/2, 0)
      And full_quarter * p = point(-1, 0, 0)
  */
  describe('given a point and two z rotation matrices', () => {
    let p: Tuple
    let halfQuarter: Matrix
    let fullQuarter: Matrix

    beforeEach(() => {
      p = Tuple.point(0, 1, 0)
      halfQuarter = Matrix.rotationZ(Math.PI / 4)
      fullQuarter = Matrix.rotationZ(Math.PI / 2)
    })

    describe('when multiplying halfQuarter by p', () => {
      it('should return the expected point', () => {
        expect(halfQuarter.mul(p)).to.equal(
          Tuple.point(-Math.sqrt(2) / 2, Math.sqrt(2) / 2, 0)
        )
      })
    })

    describe('when multiplying fullQuarter by p', () => {
      it('should return the expected point', () => {
        expect(fullQuarter.mul(p)).to.equal(Tuple.point(-1, 0, 0))
      })
    })
  })

  describe('given a shearing matrix and a point', () => {
    let transform: Matrix
    let p: Tuple

    beforeEach(() => {
      p = Tuple.point(2, 3, 4)
    })

    /*
    Scenario: A shearing transformation moves x in proportion to y
      Given transform ← shearing(1, 0, 0, 0, 0, 0)
        And p ← point(2, 3, 4)
      Then transform * p = point(5, 3, 4)
    */
    describe('when shearing x in proportion to y', () => {
      beforeEach(() => {
        transform = Matrix.shearing(1, 0, 0, 0, 0, 0)
      })

      it('should return the expected point', () => {
        expect(transform.mul(p)).to.equal(Tuple.point(5, 3, 4))
      })
    })

    /*
      Scenario: A shearing transformation moves x in proportion to z
        Given transform ← shearing(0, 1, 0, 0, 0, 0)
          And p ← point(2, 3, 4)
        Then transform * p = point(6, 3, 4)
    */
    describe('when shearing x in proportion to z', () => {
      beforeEach(() => {
        transform = Matrix.shearing(0, 1, 0, 0, 0, 0)
      })

      it('should return the expected point', () => {
        expect(transform.mul(p)).to.equal(Tuple.point(6, 3, 4))
      })
    })

    /*
      Scenario: A shearing transformation moves y in proportion to x
        Given transform ← shearing(0, 0, 1, 0, 0, 0)
          And p ← point(2, 3, 4)
        Then transform * p = point(2, 5, 4)
    */
    describe('when shearing y in proportion to x', () => {
      beforeEach(() => {
        transform = Matrix.shearing(0, 0, 1, 0, 0, 0)
      })

      it('should return the expected point', () => {
        expect(transform.mul(p)).to.equal(Tuple.point(2, 5, 4))
      })
    })

    /*
      Scenario: A shearing transformation moves y in proportion to z
        Given transform ← shearing(0, 0, 0, 1, 0, 0)
          And p ← point(2, 3, 4)
        Then transform * p = point(2, 7, 4)
    */
    describe('when shearing y in proportion to z', () => {
      beforeEach(() => {
        transform = Matrix.shearing(0, 0, 0, 1, 0, 0)
      })

      it('should return the expected point', () => {
        expect(transform.mul(p)).to.equal(Tuple.point(2, 7, 4))
      })
    })

    /*
      Scenario: A shearing transformation moves z in proportion to x
        Given transform ← shearing(0, 0, 0, 0, 1, 0)
          And p ← point(2, 3, 4)
        Then transform * p = point(2, 3, 6)
    */
    describe('when shearing z in proportion to x', () => {
      beforeEach(() => {
        transform = Matrix.shearing(0, 0, 0, 0, 1, 0)
      })

      it('should return the expected point', () => {
        expect(transform.mul(p)).to.equal(Tuple.point(2, 3, 6))
      })
    })

    /*
      Scenario: A shearing transformation moves z in proportion to y
        Given transform ← shearing(0, 0, 0, 0, 0, 1)
          And p ← point(2, 3, 4)
        Then transform * p = point(2, 3, 7)
    */
    describe('when shearing z in proportion to y', () => {
      beforeEach(() => {
        transform = Matrix.shearing(0, 0, 0, 0, 0, 1)
      })

      it('should return the expected point', () => {
        expect(transform.mul(p)).to.equal(Tuple.point(2, 3, 7))
      })
    })
  })

  /*
    Scenario: Individual transformations are applied in sequence
      Given p ← point(1, 0, 1)
        And A ← rotation_x(π / 2)
        And B ← scaling(5, 5, 5)
        And C ← translation(10, 5, 7)
      # apply rotation first
      When p2 ← A * p
      Then p2 = point(1, -1, 0)
      # then apply scaling
      When p3 ← B * p2
      Then p3 = point(5, -5, 0)
      # then apply translation
      When p4 ← C * p3
      Then p4 = point(15, 0, 7)
  */
  describe('given a point and three matrices', () => {
    let p: Tuple
    let A: Matrix
    let B: Matrix
    let C: Matrix

    beforeEach(() => {
      p = Tuple.point(1, 0, 1)
      A = Matrix.rotationX(Math.PI / 2)
      B = Matrix.scaling(5, 5, 5)
      C = Matrix.translation(10, 5, 7)
    })

    describe('when applying the transformations in sequence', () => {
      let p2: Tuple
      let p3: Tuple
      let p4: Tuple

      beforeEach(() => {
        p2 = A.mul(p)
        p3 = B.mul(p2)
        p4 = C.mul(p3)
      })

      it('should return the expected point', () => {
        expect(p2).to.equal(Tuple.point(1, -1, 0))
        expect(p3).to.equal(Tuple.point(5, -5, 0))
        expect(p4).to.equal(Tuple.point(15, 0, 7))
      })
    })
  })

  describe('chainable transformations', () => {
    it('should be applied in reverse order', () => {
      const p = Tuple.point(1, 0, 1)
      const T = Matrix.transformation()
        .rotateX(Math.PI / 2)
        .scale(5, 5, 5)
        .translate(10, 5, 7)
      expect(T.mul(p)).to.equal(Tuple.point(15, 0, 7))
    })
  })
})
