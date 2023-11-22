import { Assertion } from 'chai'

import '../../lib/matrix.spec.js'
import '../../lib/canvas.spec.js'
import '../../lib/tuple.spec.js'
import '../../lib/util/equal.spec.js'
import { Matrix } from '../../lib/matrix.js'
import { Tuple } from '../../lib/tuple.js'
import equal from '../../lib/util/equal.js'

Assertion.overwriteMethod('equal', (_super) => {
  return function (this: typeof Assertion, ...args: unknown[]) {
    let obj = this._obj

    if (
      (obj instanceof Tuple && args[0] instanceof Tuple) ||
      (obj instanceof Matrix && args[0] instanceof Matrix)
    ) {
      obj = obj as { equals(other: Tuple | Matrix): boolean }
      this.assert(
        obj.equals(args[0]),
        `expected ${obj} to equal ${args[0]}`,
        `expected ${obj} to not equal ${args[0]}`,
        obj,
        args[0],
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
