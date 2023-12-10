import { expect } from 'chai'

import { Intersection, IntersectionCollection } from './intersection.js'
import { Sphere } from './sphere.js'

describe('Intersection', () => {
  describe('constructor()', () => {
    // Scenario: An intersection encapsulates t and object
    //   Given s ← sphere()
    //   When i ← intersection(3.5, s)
    //   Then i.t = 3.5
    //     And i.object = s
    it('should encapsulate t and object', () => {
      const s = new Sphere()
      const i = new Intersection(3.5, s)
      expect(i.t).to.equal(3.5)
      expect(i.object).to.equal(s)
    })
  })
})

describe('IntersectionCollection', () => {
  describe('constructor()', () => {
    // Scenario: Aggregating intersections
    //   Given s ← sphere()
    //     And i1 ← intersection(1, s)
    //     And i2 ← intersection(2, s)
    //   When xs ← intersections(i1, i2)
    //   Then xs.count = 2
    //     And xs[0].t = 1
    //     And xs[1].t = 2
    it('should aggregate intersections', () => {
      const s = new Sphere()
      const i1 = new Intersection(1, s)
      const i2 = new Intersection(2, s)
      const xs = new IntersectionCollection(i1, i2)
      expect(xs).to.have.length(2)
      expect(xs.at(0).t).to.equal(1)
      expect(xs.at(1).t).to.equal(2)
    })
  })

  describe('hit()', () => {
    // Scenario: The hit, when all intersections have positive t
    //   Given s ← sphere()
    //     And i1 ← intersection(1, s)
    //     And i2 ← intersection(2, s)
    //     And xs ← intersections(i2, i1)
    //   When i ← hit(xs)
    //   Then i = i1
    it('should return lowest nonnegative intersection when all intersections have positive t', () => {
      const s = new Sphere()
      const i1 = new Intersection(1, s)
      const i2 = new Intersection(2, s)
      const xs = new IntersectionCollection(i2, i1)
      const i = xs.hit()
      expect(i).to.equal(i1)
    })

    // Scenario: The hit, when some intersections have negative t
    //   Given s ← sphere()
    //     And i1 ← intersection(-1, s)
    //     And i2 ← intersection(1, s)
    //     And xs ← intersections(i2, i1)
    //   When i ← hit(xs)
    //   Then i = i2
    it('should return lowest nonnegative intersection when some intersections have negative t', () => {
      const s = new Sphere()
      const i1 = new Intersection(-1, s)
      const i2 = new Intersection(1, s)
      const xs = new IntersectionCollection(i2, i1)
      const i = xs.hit()
      expect(i).to.equal(i2)
    })

    // Scenario: The hit, when all intersections have negative t
    //   Given s ← sphere()
    //     And i1 ← intersection(-2, s)
    //     And i2 ← intersection(-1, s)
    //     And xs ← intersections(i2, i1)
    //   When i ← hit(xs)
    //   Then i is nothing
    it('should return null when all intersections have negative t', () => {
      const s = new Sphere()
      const i1 = new Intersection(-2, s)
      const i2 = new Intersection(-1, s)
      const xs = new IntersectionCollection(i2, i1)
      const i = xs.hit()
      expect(i).to.be.null
    })

    // Scenario: The hit is always the lowest nonnegative intersection
    //   Given s ← sphere()
    //   And i1 ← intersection(5, s)
    //   And i2 ← intersection(7, s)
    //   And i3 ← intersection(-3, s)
    //   And i4 ← intersection(2, s)
    //   And xs ← intersections(i1, i2, i3, i4)
    // When i ← hit(xs)
    // Then i = i4
    it('should always return the lowest nonnegative intersection', () => {
      const s = new Sphere()
      const i1 = new Intersection(5, s)
      const i2 = new Intersection(7, s)
      const i3 = new Intersection(-3, s)
      const i4 = new Intersection(2, s)
      const xs = new IntersectionCollection(i1, i2, i3, i4)
      const i = xs.hit()
      expect(i).to.equal(i4)
    })
  })
})
