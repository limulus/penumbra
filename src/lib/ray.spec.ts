import '../test/setup.js'

import { expect } from 'chai'
import { describe, it } from 'vitest'

import { Matrix } from './matrix.js'
import { Ray } from './ray.js'
import { Tuple } from './tuple.js'

describe('Ray', () => {
  describe('constructor', () => {
    // Scenario: Creating and querying a ray
    //   Given origin ← point(1, 2, 3)
    //     And direction ← vector(4, 5, 6)
    //   When r ← ray(origin, direction)
    //   Then r.origin = origin
    //     And r.direction = direction
    it('should create a ray with the given origin and direction', () => {
      const origin = Tuple.point(1, 2, 3)
      const direction = Tuple.vector(4, 5, 6)
      const ray = new Ray(origin, direction)
      expect(ray.origin).to.equal(origin)
      expect(ray.direction).to.equal(direction)
    })
  })

  describe('position()', () => {
    // Scenario: Computing a point from a distance
    //   Given r ← ray(point(2, 3, 4), vector(1, 0, 0))
    //   Then position(r, 0) = point(2, 3, 4)
    //     And position(r, 1) = point(3, 3, 4)
    //     And position(r, -1) = point(1, 3, 4)
    //     And position(r, 2.5) = point(4.5, 3, 4)
    it('should compute a point from a distance', () => {
      const r = new Ray(Tuple.point(2, 3, 4), Tuple.vector(1, 0, 0))
      expect(r.position(0)).to.equal(Tuple.point(2, 3, 4))
      expect(r.position(1)).to.equal(Tuple.point(3, 3, 4))
      expect(r.position(-1)).to.equal(Tuple.point(1, 3, 4))
      expect(r.position(2.5)).to.equal(Tuple.point(4.5, 3, 4))
    })
  })

  describe('transform()', () => {
    // Scenario: Translating a ray
    //   Given r ← ray(point(1, 2, 3), vector(0, 1, 0))
    //     And m ← translation(3, 4, 5)
    //   When r2 ← transform(r, m)
    //   Then r2.origin = point(4, 6, 8)
    //     And r2.direction = vector(0, 1, 0)
    it('should translate a ray', () => {
      const r = new Ray(Tuple.point(1, 2, 3), Tuple.vector(0, 1, 0))
      const m = Matrix.translation(3, 4, 5)
      const r2 = r.transform(m)
      expect(r2.origin).to.equal(Tuple.point(4, 6, 8))
      expect(r2.direction).to.equal(Tuple.vector(0, 1, 0))
    })

    // Scenario: Scaling a ray
    //   Given r ← ray(point(1, 2, 3), vector(0, 1, 0))
    //     And m ← scaling(2, 3, 4)
    //   When r2 ← transform(r, m)
    //   Then r2.origin = point(2, 6, 12)
    //     And r2.direction = vector(0, 3, 0)
    it('should scale a ray', () => {
      const r = new Ray(Tuple.point(1, 2, 3), Tuple.vector(0, 1, 0))
      const m = Matrix.scaling(2, 3, 4)
      const r2 = r.transform(m)
      expect(r2.origin).to.equal(Tuple.point(2, 6, 12))
      expect(r2.direction).to.equal(Tuple.vector(0, 3, 0))
    })
  })
})
