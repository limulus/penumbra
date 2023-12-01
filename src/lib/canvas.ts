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
    let offset = (Math.floor(y) * this.width + Math.floor(x)) * 3
    return Tuple.color(this.data[offset], this.data[++offset], this.data[++offset])
  }

  writePixel(x: number, y: number, color: Tuple) {
    const { data } = this
    let offset = (Math.floor(y) * this.width + Math.floor(x)) * 3
    data[offset] = color.red
    data[++offset] = color.green
    data[++offset] = color.blue
  }

  toImageData(): ImageData {
    const image = new ImageData(this.width, this.height)

    for (let i = 0, j = 0; i < this.data.length; i += 3, j += 4) {
      image.data[j] = this.data[i] * 255
      image.data[j + 1] = this.data[i + 1] * 255
      image.data[j + 2] = this.data[i + 2] * 255
      image.data[j + 3] = 255
    }

    return image
  }

  toPPM() {
    let buffer =
      outdent`
        P3
        ${this.width} ${this.height}
        255
      ` + '\n'

    for (let i = 0; i < this.data.length; i++) {
      buffer += `${floatToByte(this.data[i])} `
      if (i % 15 === 14) {
        buffer = buffer.trimEnd() + '\n'
      }
    }

    return buffer.trimEnd() + '\n'
  }
}

const floatToByte = (value: number) => {
  value = Math.round(value * 255)
  value = Math.min(value, 255)
  value = Math.max(value, 0)
  return value
}
