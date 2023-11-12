import type { Suite, Func } from 'mocha'

type SuiteFunction = (this: Suite) => void

export const Feature = (message: string, fn: SuiteFunction) =>
  describe(`Feature: ${message}`, fn)
export const Scenario = (message: string, fn: SuiteFunction) =>
  describe(`Scenario: ${message}`, fn)
export const Given = (message: string, fn: SuiteFunction) =>
  describe(`Given ${message}`, fn)
export const When = (message: string, fn: SuiteFunction) => describe(`When ${message}`, fn)
export const Then = (message: string, fn: Func) => it(`Then ${message}`, fn)
export const And = (message: string, fn: Func) => it(`And ${message}`, fn)
