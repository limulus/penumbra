import '../test/setup.js'

import { expect } from 'chai'
import { describe, it } from 'vitest'

import { Matrix } from './matrix.js'
import { Ray } from './ray.js'
import { Sphere } from './sphere.js'
import { Tuple } from './tuple.js'

describe('Sphere', () => {
  describe('intersect()', () => {
    // Scenario: A ray intersects a sphere at two points
    //  Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    //    And s ← sphere()
    //  When xs ← intersect(s, r)
    //  Then xs.count = 2
    //    And xs[0] = 4.0
    //    And xs[1] = 6.0
    it('should return the two points of intersection along the ray', () => {
      const r = new Ray(Tuple.point(0, 0, -5), Tuple.vector(0, 0, 1))
      const s = new Sphere()
      const xs = s.intersect(r)
      expect(xs).to.have.length(2)
      expect(xs.at(0).t).to.equal(4)
      expect(xs.at(1).t).to.equal(6)
    })

    // Scenario: A ray intersects a sphere at a tangent
    //   Given r ← ray(point(0, 1, -5), vector(0, 0, 1))
    //     And s ← sphere()
    //   When xs ← intersect(s, r)
    //   Then xs.count = 2
    //     And xs[0] = 5.0
    //     And xs[1] = 5.0
    it('should return the same two points of intersection when the ray glances off the sphere', () => {
      const r = new Ray(Tuple.point(0, 1, -5), Tuple.vector(0, 0, 1))
      const s = new Sphere()
      const xs = s.intersect(r)
      expect(xs).to.have.length(2)
      expect(xs.at(0).t).to.equal(5)
      expect(xs.at(1).t).to.equal(5)
    })

    // Scenario: A ray misses a sphere
    //   Given r ← ray(point(0, 2, -5), vector(0, 0, 1))
    //     And s ← sphere()
    //   When xs ← intersect(s, r)
    //   Then xs.count = 0
    it('should return no intersections when the ray does not intersect the sphere', () => {
      const r = new Ray(Tuple.point(0, 2, -5), Tuple.vector(0, 0, 1))
      const s = new Sphere()
      const xs = s.intersect(r)
      expect(xs).to.have.length(0)
    })

    // Scenario: A ray originates inside a sphere
    //   Given r ← ray(point(0, 0, 0), vector(0, 0, 1))
    //     And s ← sphere()
    //   When xs ← intersect(s, r)
    //   Then xs.count = 2
    //     And xs[0] = -1.0
    //     And xs[1] = 1.0
    it('should return the two points of intersection when the ray originates inside the sphere', () => {
      const r = new Ray(Tuple.point(0, 0, 0), Tuple.vector(0, 0, 1))
      const s = new Sphere()
      const xs = s.intersect(r)
      expect(xs).to.have.length(2)
      expect(xs.at(0).t).to.equal(-1)
      expect(xs.at(1).t).to.equal(1)
    })

    // Scenario: Intersect sets the object on the intersection
    //   Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    //     And s ← sphere()
    //   When xs ← intersect(s, r)
    //   Then xs.count = 2
    //     And xs[0].object = s
    //     And xs[1].object = s
    it('should set the object on the intersection', () => {
      const r = new Ray(Tuple.point(0, 0, -5), Tuple.vector(0, 0, 1))
      const s = new Sphere()
      const xs = s.intersect(r)
      expect(xs).to.have.length(2)
      expect(xs.at(0).object).to.equal(s)
      expect(xs.at(1).object).to.equal(s)
    })

    // Scenario: Intersecting a scaled sphere with a ray
    //   Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    //     And s ← sphere()
    //   When set_transform(s, scaling(2, 2, 2))
    //     And xs ← intersect(s, r)
    //   Then xs.count = 2
    //     And xs[0].t = 3
    //     And xs[1].t = 7
    it('should return any points of intersection when the ray intersects a scaled sphere', () => {
      const r = new Ray(Tuple.point(0, 0, -5), Tuple.vector(0, 0, 1))
      const s = new Sphere()
      s.transform = Matrix.scaling(2, 2, 2)
      const xs = s.intersect(r)
      expect(xs).to.have.length(2)
      expect(xs.at(0).t).to.equal(3)
      expect(xs.at(1).t).to.equal(7)
    })

    // Scenario: Intersecting a translated sphere with a ray
    //   Given r ← ray(point(0, 0, -5), vector(0, 0, 1))
    //     And s ← sphere()
    //   When set_transform(s, translation(5, 0, 0))
    //     And xs ← intersect(s, r)
    //   Then xs.count = 0
    it('should return any points of intersection when the ray intersects a translated sphere', () => {
      const r = new Ray(Tuple.point(0, 0, -5), Tuple.vector(0, 0, 1))
      const s = new Sphere()
      s.transform = Matrix.translation(5, 0, 0)
      const xs = s.intersect(r)
      expect(xs).to.have.length(0)
    })
  })

  describe('transform', () => {
    // Scenario: A sphere's default transformation
    //   Given s ← sphere()
    //   Then s.transform = identity_matrix
    it('should have an identity matrix as its default transformation', () => {
      const s = new Sphere()
      expect(s.transform).to.equal(Matrix.identity(4))
    })

    // Scenario: Changing a sphere's transformation
    //   Given s ← sphere()
    //     And t ← translation(2, 3, 4)
    //   When set_transform(s, t)
    //   Then s.transform = t
    it("should change the sphere's transformation", () => {
      const s = new Sphere()
      const t = Matrix.translation(2, 3, 4)
      s.transform = t
      expect(s.transform).to.equal(t)
    })
  })
})
