import { expect } from 'chai'

import { Tuple } from './Tuple.js'
import { Feature, Scenario, Then, And, Given } from '../gherkin'

Feature('Tuple', () => {
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
        expect(p).to.equal(new Tuple(4, -4, 3, 1))
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
        expect(v).to.equal(new Tuple(4, -4, 3, 0))
      })
    })
  })

  /*
  Scenario: Adding two tuples
  Given a1 ← tuple(3, -2, 5, 1)
    And a2 ← tuple(-2, 3, 1, 0)
   Then a1 + a2 = tuple(1, 1, 6, 1)
  */
  Scenario('Adding two tuples', () => {
    Given('a1 ← tuple(3, -2, 5, 1)', () => {
      const a1 = new Tuple(3, -2, 5, 1)

      And('a2 ← tuple(-2, 3, 1, 0)', () => {
        const a2 = new Tuple(-2, 3, 1, 0)

        Then('a1 + a2 = tuple(1, 1, 6, 1)', () => {
          expect(a1.add(a2)).to.equal(new Tuple(1, 1, 6, 1))
        })
      })
    })
  })

  /*
  Scenario: Subtracting two points
    Given p1 ← point(3, 2, 1)
      And p2 ← point(5, 6, 7)
    Then p1 - p2 = vector(-2, -4, -6)
  */
  Scenario('Subtracting two points', () => {
    Given('p1 ← point(3, 2, 1)', () => {
      const p1 = Tuple.point(3, 2, 1)

      And('p2 ← point(5, 6, 7)', () => {
        const p2 = Tuple.point(5, 6, 7)

        Then('p1 - p2 = vector(-2, -4, -6)', () => {
          expect(p1.sub(p2)).to.equal(Tuple.vector(-2, -4, -6))
        })
      })
    })
  })

  /*
  Scenario: Subtracting a vector from a point
    Given p ← point(3, 2, 1)
      And v ← vector(5, 6, 7)
    Then p - v = point(-2, -4, -6)
  */
  Scenario('Subtracting a vector from a point', () => {
    Given('p ← point(3, 2, 1)', () => {
      const p = Tuple.point(3, 2, 1)

      And('v ← vector(5, 6, 7)', () => {
        const v = Tuple.vector(5, 6, 7)

        Then('p - v = point(-2, -4, -6)', () => {
          expect(p.sub(v)).to.equal(Tuple.point(-2, -4, -6))
        })
      })
    })
  })

  /*
  Scenario: Subtracting two vectors
    Given v1 ← vector(3, 2, 1)
      And v2 ← vector(5, 6, 7)
    Then v1 - v2 = vector(-2, -4, -6)
  */
  Scenario('Subtracting two vectors', () => {
    Given('v1 ← vector(3, 2, 1)', () => {
      const v1 = Tuple.vector(3, 2, 1)

      And('v2 ← vector(5, 6, 7)', () => {
        const v2 = Tuple.vector(5, 6, 7)

        Then('v1 - v2 = vector(-2, -4, -6)', () => {
          expect(v1.sub(v2)).to.equal(Tuple.vector(-2, -4, -6))
        })
      })
    })
  })

  /*
  Scenario: Subtracting a vector from the zero vector
    Given zero ← vector(0, 0, 0)
      And v ← vector(1, -2, 3)
    Then zero - v = vector(-1, 2, -3)
  */
  Scenario('Subtracting a vector from the zero vector', () => {
    Given('zero ← vector(0, 0, 0)', () => {
      const zero = Tuple.vector(0, 0, 0)

      And('v ← vector(1, -2, 3)', () => {
        const v = Tuple.vector(1, -2, 3)

        Then('zero - v = vector(-1, 2, -3)', () => {
          expect(zero.sub(v)).to.equal(Tuple.vector(-1, 2, -3))
        })
      })
    })
  })

  /*
  Scenario: Negating a tuple
    Given a ← tuple(1, -2, 3, -4)
    Then -a = tuple(-1, 2, -3, 4)
  */
  Scenario('Negating a tuple', () => {
    Given('a ← tuple(1, -2, 3, -4)', () => {
      const a = new Tuple(1, -2, 3, -4)

      Then('-a = tuple(-1, 2, -3, 4)', () => {
        expect(a.negate()).to.equal(new Tuple(-1, 2, -3, 4))
      })
    })
  })

  /*
  Scenario: Multiplying a tuple by a scalar
    Given a ← tuple(1, -2, 3, -4)
    Then a * 3.5 = tuple(3.5, -7, 10.5, -14)
  */
  Scenario('Multiplying a tuple by a scalar', () => {
    Given('a ← tuple(1, -2, 3, -4)', () => {
      const a = new Tuple(1, -2, 3, -4)

      Then('a * 3.5 = tuple(3.5, -7, 10.5, -14)', () => {
        expect(a.mul(3.5)).to.equal(new Tuple(3.5, -7, 10.5, -14))
      })
    })
  })

  /*
  Scenario: Multiplying a tuple by a fraction
    Given a ← tuple(1, -2, 3, -4)
    Then a * 0.5 = tuple(0.5, -1, 1.5, -2)
  */
  Scenario('Multiplying a tuple by a fraction', () => {
    Given('a ← tuple(1, -2, 3, -4)', () => {
      const a = new Tuple(1, -2, 3, -4)

      Then('a * 0.5 = tuple(0.5, -1, 1.5, -2)', () => {
        expect(a.mul(0.5)).to.equal(new Tuple(0.5, -1, 1.5, -2))
      })
    })
  })

  /*
  Scenario: Dividing a tuple by a scalar
    Given a ← tuple(1, -2, 3, -4)
    Then a / 2 = tuple(0.5, -1, 1.5, -2)
  */
  Scenario('Dividing a tuple by a scalar', () => {
    Given('a ← tuple(1, -2, 3, -4)', () => {
      const a = new Tuple(1, -2, 3, -4)

      Then('a / 2 = tuple(0.5, -1, 1.5, -2)', () => {
        expect(a.div(2)).to.equal(new Tuple(0.5, -1, 1.5, -2))
      })
    })
  })

  /*
  Scenario: Computing the magnitude of vector(1, 0, 0)
    Given v ← vector(1, 0, 0)
    Then magnitude(v) = 1
  */
  Scenario('Computing the magnitude of vector(1, 0, 0)', () => {
    Given('v ← vector(1, 0, 0)', () => {
      const v = Tuple.vector(1, 0, 0)

      Then('magnitude(v) = 1', () => {
        expect(v.magnitude()).to.equal(1)
      })
    })
  })

  /*
  Scenario: Computing the magnitude of vector(0, 1, 0)
    Given v ← vector(0, 1, 0)
    Then magnitude(v) = 1
  */
  Scenario('Computing the magnitude of vector(0, 1, 0)', () => {
    Given('v ← vector(0, 1, 0)', () => {
      const v = Tuple.vector(0, 1, 0)

      Then('magnitude(v) = 1', () => {
        expect(v.magnitude()).to.equal(1)
      })
    })
  })

  /*
  Scenario: Computing the magnitude of vector(0, 0, 1)
    Given v ← vector(0, 0, 1)
    Then magnitude(v) = 1
  */
  Scenario('Computing the magnitude of vector(0, 0, 1)', () => {
    Given('v ← vector(0, 0, 1)', () => {
      const v = Tuple.vector(0, 0, 1)

      Then('magnitude(v) = 1', () => {
        expect(v.magnitude()).to.equal(1)
      })
    })
  })

  /*
  Scenario: Computing the magnitude of vector(1, 2, 3)
    Given v ← vector(1, 2, 3)
    Then magnitude(v) = √14
  */
  Scenario('Computing the magnitude of vector(1, 2, 3)', () => {
    Given('v ← vector(1, 2, 3)', () => {
      const v = Tuple.vector(1, 2, 3)

      Then('magnitude(v) = √14', () => {
        expect(v.magnitude()).to.equal(Math.sqrt(14))
      })
    })
  })

  /*
  Scenario: Computing the magnitude of vector(-1, -2, -3)
    Given v ← vector(-1, -2, -3)
    Then magnitude(v) = √14
  */
  Scenario('Computing the magnitude of vector(-1, -2, -3)', () => {
    Given('v ← vector(-1, -2, -3)', () => {
      const v = Tuple.vector(-1, -2, -3)

      Then('magnitude(v) = √14', () => {
        expect(v.magnitude()).to.equal(Math.sqrt(14))
      })
    })
  })
})
