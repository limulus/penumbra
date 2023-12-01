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
    const offset = this.pixelOffset(x, y)
    return Tuple.color(this.data[offset + 0], this.data[offset + 1], this.data[offset + 2])
  }

  private pixelOffset(x: number, y: number): number {
    return (Math.floor(y) * this.width + Math.floor(x)) * 3
  }

  writePixel(x: number, y: number, color: Tuple): void
  writePixel(x: number, y: number, red: number, green: number, blue: number): void
  writePixel(
    x: number,
    y: number,
    colorOrRed: number | Tuple,
    green?: number,
    blue?: number
  ): void {
    if (colorOrRed instanceof Tuple)
      return this.writePixel(x, y, colorOrRed.red, colorOrRed.green, colorOrRed.blue)
    const offset = this.pixelOffset(x, y)
    this.data[offset + 0] = colorOrRed
    this.data[offset + 1] = green!
    this.data[offset + 2] = blue!
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
