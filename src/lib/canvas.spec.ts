import { expect } from 'chai'
import { outdent } from 'outdent'

import { Canvas } from './canvas.js'
import { Tuple } from './tuple.js'
import { Feature, Scenario, Then, And, Given, When } from '../gherkin.js'

Feature('Canvas', () => {
  /*
  Scenario: Creating a canvas
    Given c ← canvas(10, 20)
    Then c.width = 10
      And c.height = 20
      And every pixel of c is color(0, 0, 0)
  */
  Scenario('Creating a canvas', () => {
    let c: Canvas

    Given('c ← canvas(10, 20)', () => {
      c = new Canvas(10, 20)
    })

    Then('c.width = 10', () => {
      expect(c.width).to.equal(10)
    })

    And('c.height = 20', () => {
      expect(c.height).to.equal(20)
    })

    And('every pixel of c is color(0, 0, 0)', () => {
      const black = Tuple.color(0, 0, 0)

      for (let x = 0; x < c.width; x++) {
        for (let y = 0; y < c.height; y++) {
          expect(c.pixelAt(x, y)).to.equal(black)
        }
      }
    })
  })

  /*
  Scenario: Writing pixels to a canvas
    Given c ← canvas(10, 20)
      And red ← color(1, 0, 0)
    When write_pixel(c, 2, 3, red)
    Then pixel_at(c, 2, 3) = red
  */
  Scenario('Writing pixels to a canvas', () => {
    let c: Canvas
    let red: Tuple

    Given('c ← canvas(10, 20)', () => {
      c = new Canvas(10, 20)
    })

    And('red ← color(1, 0, 0)', () => {
      red = Tuple.color(1, 0, 0)
    })

    When('write_pixel(c, 2, 3, red)', () => {
      c.writePixel(2, 3, red)
    })

    Then('pixel_at(c, 2, 3) = red', () => {
      expect(c.pixelAt(2, 3)).to.equal(red)
    })
  })

  /*
  Scenario: Constructing the PPM header
    Given c ← canvas(5, 3)
    When ppm ← canvas_to_ppm(c)
    Then lines 1-3 of ppm are
      """
      P3
      5 3
      255
      """
  */
  Scenario('Constructing the PPM header', () => {
    let c: Canvas
    let ppm: string

    Given('c ← canvas(5, 3)', () => {
      c = new Canvas(5, 3)
    })

    When('ppm ← canvas_to_ppm(c)', () => {
      ppm = c.toPPM()
    })

    Then('lines 1-3 of ppm have the expected header', () => {
      expect(ppm).to.match(/^P3\n5 3\n255\n/)
    })
  })

  /*
  Scenario: Constructing the PPM pixel data
    Given c ← canvas(5, 3)
      And c1 ← color(1.5, 0, 0)
      And c2 ← color(0, 0.5, 0)
      And c3 ← color(-0.5, 0, 1)
    When write_pixel(c, 0, 0, c1)
      And write_pixel(c, 2, 1, c2)
      And write_pixel(c, 4, 2, c3)
      And ppm ← canvas_to_ppm(c)
    Then lines 4-6 of ppm are
      """
      255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
      0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
      0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
      """
  */
  Scenario('Constructing the PPM pixel data', () => {
    let c: Canvas
    let c1: Tuple
    let c2: Tuple
    let c3: Tuple
    let ppm: string

    Given('c ← canvas(5, 3)', () => {
      c = new Canvas(5, 3)
    })

    And('c1 ← color(1.5, 0, 0)', () => {
      c1 = Tuple.color(1.5, 0, 0)
    })

    And('c2 ← color(0, 0.5, 0)', () => {
      c2 = Tuple.color(0, 0.5, 0)
    })

    And('c3 ← color(-0.5, 0, 1)', () => {
      c3 = Tuple.color(-0.5, 0, 1)
    })

    When('write_pixel(c, 0, 0, c1)', () => {
      c.writePixel(0, 0, c1)
    })

    And('write_pixel(c, 2, 1, c2)', () => {
      c.writePixel(2, 1, c2)
    })

    And('write_pixel(c, 4, 2, c3)', () => {
      c.writePixel(4, 2, c3)
    })

    And('ppm ← canvas_to_ppm(c)', () => {
      ppm = c.toPPM()
    })

    Then('lines 4-6 of ppm have the expected pixel data', () => {
      expect(ppm.split('\n').slice(3, 6).join('\n')).to.equal(outdent`
        255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
        0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
      `)
    })
  })

  /*
  Scenario: Splitting long lines in PPM files
    Given c ← canvas(10, 2)
    When every pixel of c is set to color(1, 0.8, 0.6)
      And ppm ← canvas_to_ppm(c)
    Then lines 4-7 of ppm are
      """
      255 204 153 255 204 153 255 204 153 255 204 153 255 204 153
      255 204 153 255 204 153 255 204 153 255 204 153 255 204 153
      255 204 153 255 204 153 255 204 153 255 204 153 255 204 153
      255 204 153 255 204 153 255 204 153 255 204 153 255 204 153
      """
  */
  Scenario('Splitting long lines in PPM files', () => {
    let c: Canvas
    let ppm: string

    Given('c ← canvas(10, 2)', () => {
      c = new Canvas(10, 2)
    })

    When('every pixel of c is set to color(1, 0.8, 0.6)', () => {
      const color = Tuple.color(1, 0.8, 0.6)

      for (let x = 0; x < c.width; x++) {
        for (let y = 0; y < c.height; y++) {
          c.writePixel(x, y, color)
        }
      }
    })

    And('ppm ← canvas_to_ppm(c)', () => {
      ppm = c.toPPM()
      console.log(ppm)
    })

    Then('lines 4-7 of ppm have the expected pixel data', () => {
      expect(ppm.split('\n').slice(3, 7).join('\n')).to.equal(outdent`
        255 204 153 255 204 153 255 204 153 255 204 153 255 204 153
        255 204 153 255 204 153 255 204 153 255 204 153 255 204 153
        255 204 153 255 204 153 255 204 153 255 204 153 255 204 153
        255 204 153 255 204 153 255 204 153 255 204 153 255 204 153
      `)
    })
  })

  /*
  Scenario: PPM files are terminated by a newline character
    Given c ← canvas(5, 3)
    When ppm ← canvas_to_ppm(c)
    Then ppm ends with a newline character
  */
  Scenario('PPM files are terminated by a newline character', () => {
    let c: Canvas
    let ppm: string

    Given('c ← canvas(5, 3)', () => {
      c = new Canvas(5, 3)
    })

    When('ppm ← canvas_to_ppm(c)', () => {
      ppm = c.toPPM()
    })

    Then('ppm ends with a newline character', () => {
      expect(ppm.endsWith('\n')).to.be.true
    })
  })
})
