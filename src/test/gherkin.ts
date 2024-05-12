import { Suite, TestFunction, describe, it } from 'vitest'

let prevKeyword: 'Given' | 'Then' = 'Given'

type SuiteFunction = (this: Suite) => void

export const Feature = (message: string, fn: SuiteFunction) =>
  describe(`Feature: ${message}`, fn)

export const Scenario = (message: string, fn: SuiteFunction) => {
  describe(`Scenario: ${message}`, fn)
  prevKeyword = 'Given'
}

export const Given = (message: string, fn: SuiteFunction) => {
  describe(`Given ${message}`, fn)
  prevKeyword = 'Given'
}

export const When = (message: string, fn: SuiteFunction) => describe(`When ${message}`, fn)

export const Then = (message: string, fn: TestFunction) => {
  it(`Then ${message}`, fn)
  prevKeyword = 'Then'
}

export const And = (message: string, fn: TestFunction | SuiteFunction) => {
  switch (prevKeyword) {
    case 'Given':
      describe(`And ${message}`, fn as SuiteFunction)
      break
    case 'Then':
      it(`And ${message}`, fn as TestFunction)
      break
    default:
      throw new Error(`Unexpected previous keyword for And call: ${prevKeyword}`)
  }
}
