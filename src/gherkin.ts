import type { Suite, Func } from 'mocha'

type SuiteFunction = (this: Suite) => void

type PrevKeyword = 'Given' | 'Then'
const prevKeywordSym = Symbol('prevKeyword')
declare global {
  interface BrowserMocha {
    [prevKeywordSym]: PrevKeyword
  }
}

export const Feature = (message: string, fn: SuiteFunction) =>
  describe(`Feature: ${message}`, fn)

export const Scenario = (message: string, fn: SuiteFunction) =>
  describe(`Scenario: ${message}`, fn)

export const Given = (message: string, fn: SuiteFunction) => {
  describe(`Given ${message}`, fn)
  mocha[prevKeywordSym] = 'Given'
}

export const When = (message: string, fn: SuiteFunction) => describe(`When ${message}`, fn)

export const Then = (message: string, fn: Func) => {
  it(`Then ${message}`, fn)
  mocha[prevKeywordSym] = 'Then'
}

export const And = (message: string, fn: Func | SuiteFunction) => {
  switch (mocha[prevKeywordSym]) {
    case 'Given':
      describe(`And ${message}`, fn as SuiteFunction)
      break
    case 'Then':
      it(`And ${message}`, fn as Func)
      break
    default:
      throw new Error(`Unexpected previous keyword for And call: ${mocha[prevKeywordSym]}`)
  }
}
