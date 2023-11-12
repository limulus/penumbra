import { expect } from 'chai'

import { Tuple } from './Tuple.js'
import { Feature, Scenario, Then, And, Given } from '../gherkin'

Feature('tuples', () => {
  /*
  Scenario: A tuple with w=1.0 is a point
    Given a ← tuple(4.3, -4.2, 3.1, 1.0)
    Then a.x = 4.3
      And a.y = -4.2
      And a.z = 3.1
      And a.w = 1.0
      And a is a point
      And a is not a vector
  */
  Scenario('A tuple with w=1.0 is a point', () => {
    Given('a ← tuple(4.3, -4.2, 3.1, 1.0)', () => {
      const a = new Tuple(4.3, -4.2, 3.1, 1.0)

      Then('a.x = 4.3', () => {
        expect(a).to.have.property('x', 4.3)
      })

      And('a.y = -4.2', () => {
        expect(a).to.have.property('y', -4.2)
      })

      And('a.z = 3.1', () => {
        expect(a).to.have.property('z', 3.1)
      })

      And('a.w = 1.0', () => {
        expect(a).to.have.property('w', 1.0)
      })

      And('a is a point', () => {
        expect(a.isPoint()).to.be.true
      })

      And('a is not a vector', () => {
        expect(a.isVector()).to.be.false
      })
    })
  })

  /*
  Scenario: A tuple with w=0 is a vector
    Given a ← tuple(4.3, -4.2, 3.1, 0.0)
    Then a.x = 4.3
      And a.y = -4.2
      And a.z = 3.1
      And a.w = 0.0
      And a is not a point
      And a is a vector
  */
  Scenario('A tuple with w=0 is a vector', () => {
    Given('a ← tuple(4.3, -4.2, 3.1, 0.0)', () => {
      const a = new Tuple(4.3, -4.2, 3.1, 0.0)

      Then('a.x = 4.3', () => {
        expect(a).to.have.property('x', 4.3)
      })

      And('a.y = -4.2', () => {
        expect(a).to.have.property('y', -4.2)
      })

      And('a.z = 3.1', () => {
        expect(a).to.have.property('z', 3.1)
      })

      And('a.w = 0.0', () => {
        expect(a).to.have.property('w', 0.0)
      })

      And('a is not a point', () => {
        expect(a.isPoint()).to.be.false
      })

      And('a is a vector', () => {
        expect(a.isVector()).to.be.true
      })
    })
  })

  /*
  Scenario: point() creates tuples with w=1
    Given p ← point(4, -4, 3)
    Then p = tuple(4, -4, 3, 1)
  */
  Scenario('point() creates tuples with w=1', () => {
    Given('p ← point(4, -4, 3)', () => {
      const p = Tuple.point(4, -4, 3)

      Then('p = tuple(4, -4, 3, 1)', () => {
        expect(p).to.deep.equal(new Tuple(4, -4, 3, 1))
      })
    })
  })

  /*
  Scenario: vector() creates tuples with w=0
    Given v ← vector(4, -4, 3)
    Then v = tuple(4, -4, 3, 0)
  */
  Scenario('vector() creates tuples with w=0', () => {
    Given('v ← vector(4, -4, 3)', () => {
      const v = Tuple.vector(4, -4, 3)

      Then('v = tuple(4, -4, 3, 0)', () => {
        expect(v).to.deep.equal(new Tuple(4, -4, 3, 0))
      })
    })
  })
})
