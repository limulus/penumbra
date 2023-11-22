const float32Converter = new Float32Array(1)

export function float32(n: number) {
  float32Converter[0] = n
  return float32Converter[0]
}
