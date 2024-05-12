import { expect } from 'chai'
import { describe, it } from 'vitest'

import equal from './equal.js'

describe('equal()', () => {
  describe('when comparing two numbers with a difference larger than epsilon', () => {
    it('returns false', () => {
      expect(equal(1.0, 1.00001)).to.be.false
    })
  })

  describe('when comparing two numbers with a difference smaller than epsilon', () => {
    it('returns true', () => {
      expect(equal(1.0, 1.000009)).to.be.true
    })
  })
})
