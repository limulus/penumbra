import { Assertion } from 'chai'

import { TwoDimensionalArray } from '../lib/two-dimenisonal-array.js'
import equal from '../lib/util/equal.js'

Assertion.overwriteMethod('equal', (_super) => {
  return function (this: typeof Assertion, ...args: unknown[]) {
    const obj = this._obj

    if (obj instanceof TwoDimensionalArray && obj instanceof TwoDimensionalArray) {
      const other = args[0] as TwoDimensionalArray
      this.assert(
        obj.equals(other),
        `expected ${obj} to equal ${other}`,
        `expected ${obj} to not equal ${other}`,
        obj,
        other,
        true
      )
    } else {
      _super.apply(this, args)
    }
  }
})

Assertion.addMethod('approxEqual', function (this: typeof Assertion, other: number) {
  const value = this._obj
  this.assert(
    equal(value, other),
    `expected ${value} to be approximately equal to ${other}`,
    `expected ${value} to not be approximately equal to ${other}`,
    value,
    other
  )
})

declare global {
  export namespace Chai {
    interface Assertion {
      /**
       * Asserts that the other number is approximately equal to the given number, given the
       * EPSILON value defined in equal.ts.
       * @param other The other number to compare to.
       */
      approxEqual(other: number): Assertion
    }
  }
}
