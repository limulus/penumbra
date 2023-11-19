import outdent from 'outdent'

import { Tuple } from './tuple.js'

export class Canvas {
  private readonly data: Float32Array

  constructor(
    readonly width: number,
    readonly height: number
  ) {
    this.data = new Float32Array(width * height * 3)
  }

  pixelAt(x: number, y: number) {
    let offset = (y * this.width + x) * 3
    return Tuple.color(this.data[offset], this.data[++offset], this.data[++offset])
  }

  writePixel(x: number, y: number, color: Tuple) {
    const { data } = this
    let offset = (y * this.width + x) * 3
    data[offset] = color.red
    data[++offset] = color.green
    data[++offset] = color.blue
  }

  toPPM() {
    let buffer =
      outdent`
        P3
        ${this.width} ${this.height}
        255
      ` + '\n'

    for (let i = 0; i < this.data.length; i++) {
      let value = Math.round(this.data[i] * 255)
      value = Math.min(value, 255)
      value = Math.max(value, 0)
      buffer += `${value} `

      if (i % 15 === 14) {
        buffer = buffer.trimEnd() + '\n'
      }
    }

    return buffer.trimEnd() + '\n'
  }
}
