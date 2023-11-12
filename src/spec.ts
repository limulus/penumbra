import { Assertion } from 'chai'

import './lib/Tuple.spec.js'
import './lib/equal.spec.js'
import { Tuple } from './lib/Tuple.js'

Assertion.overwriteMethod('equal', (_super) => {
  return function (this: typeof Assertion, ...args: unknown[]) {
    const obj = this._obj

    if (obj instanceof Tuple && args[0] instanceof Tuple) {
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
