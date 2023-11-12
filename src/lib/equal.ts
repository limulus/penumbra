const EPSILON = 0.00001

export default function equal(a: number, b: number) {
  return Math.abs(a - b) < EPSILON
}
