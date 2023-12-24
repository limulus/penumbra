import { expect } from 'chai'

import { TouchPad, TouchPadMoveEvent } from './touch-pad.js'

describe('TouchPad', () => {
  let target: HTMLDivElement
  let touchPad: TouchPad

  beforeEach(() => {
    target = document.createElement('div')
    target.style.position = 'absolute'
    target.style.top = '20px'
    target.style.left = '20px'
    target.style.width = '100px'
    target.style.height = '100px'
    document.body.appendChild(target)
    touchPad = new TouchPad(target)
  })

  afterEach(() => {
    target.remove()
    touchPad.disconnect()
  })

  describe('when attached element is clicked and dragged on', () => {
    it('should emit events with every movement of the mouse', () => {
      const events: TouchPadMoveEvent[] = []
      touchPad.addEventListener('touchpadmove', (event) => {
        events.push(event as TouchPadMoveEvent)
      })

      const rect = target.getBoundingClientRect()
      target.dispatchEvent(
        new MouseEvent('mousedown', {
          clientX: 0 + rect.left,
          clientY: 0 + rect.top,
          bubbles: true,
        })
      )
      target.dispatchEvent(
        new MouseEvent('mousemove', {
          clientX: 10 + rect.left,
          clientY: 10 + rect.top,
          bubbles: true,
        })
      )
      target.dispatchEvent(
        new MouseEvent('mousemove', {
          clientX: 20 + rect.left,
          clientY: 20 + rect.top,
          bubbles: true,
        })
      )
      target.dispatchEvent(
        new MouseEvent('mouseup', {
          clientX: 20 + rect.left,
          clientY: 20 + rect.top,
          bubbles: true,
        })
      )
      target.dispatchEvent(
        new MouseEvent('mousemove', {
          clientX: 30 + rect.left,
          clientY: 30 + rect.top,
          bubbles: true,
        })
      )

      expect(events).to.have.lengthOf(3)
      expect(events[0].detail).to.deep.equal({ x: 0, y: 0 })
      expect(events[1].detail).to.deep.equal({ x: 10 / 100, y: 10 / 100 })
      expect(events[2].detail).to.deep.equal({ x: 20 / 100, y: 20 / 100 })
    })

    it('should continue moving when mouse leaves the element', () => {
      const events: TouchPadMoveEvent[] = []
      touchPad.addEventListener('touchpadmove', (event) => {
        events.push(event as TouchPadMoveEvent)
      })

      const rect = target.getBoundingClientRect()
      target.dispatchEvent(
        new MouseEvent('mousedown', {
          clientX: 0 + rect.left,
          clientY: 0 + rect.top,
          bubbles: true,
        })
      )
      target.dispatchEvent(
        new MouseEvent('mousemove', {
          clientX: 10 + rect.left,
          clientY: 10 + rect.top,
          bubbles: true,
        })
      )
      target.dispatchEvent(
        new MouseEvent('mouseleave', {
          clientX: 0 + rect.left,
          clientY: 0 + rect.top,
          bubbles: true,
        })
      )
      document.dispatchEvent(new MouseEvent('mousemove', { clientX: 10, clientY: 10 }))
      document.dispatchEvent(new MouseEvent('mousemove', { clientX: 2, clientY: 2 }))
      document.dispatchEvent(new MouseEvent('mouseup', { clientX: 2, clientY: 2 }))
      document.dispatchEvent(new MouseEvent('mousemove', { clientX: 5, clientY: 5 }))

      expect(events).to.have.lengthOf(4)
      expect(events[0].detail).to.deep.equal({ x: 0, y: 0 })
      expect(events[1].detail).to.deep.equal({ x: 10 / 100, y: 10 / 100 })
      expect(events[2].detail).to.deep.equal({ x: -10 / 100, y: -10 / 100 })
      expect(events[3].detail).to.deep.equal({ x: -(20 - 2) / 100, y: -(20 - 2) / 100 })
    })
  })

  describe('when attached element is touched and dragged on', () => {
    it('should emit events with every movement of the touch', function () {
      if (!self.TouchEvent) {
        return this.skip()
      }

      const events: TouchPadMoveEvent[] = []
      touchPad.addEventListener('touchpadmove', (event) => {
        events.push(event as TouchPadMoveEvent)
      })

      const rect = target.getBoundingClientRect()
      target.dispatchEvent(
        new TouchEvent('touchstart', {
          touches: [
            new Touch({
              target,
              identifier: 0,
              clientX: 0 + rect.left,
              clientY: 0 + rect.top,
            }),
          ],
        })
      )
      target.dispatchEvent(
        new TouchEvent('touchmove', {
          touches: [
            new Touch({
              target,
              identifier: 0,
              clientX: 10 + rect.left,
              clientY: 10 + rect.top,
            }),
          ],
        })
      )
      target.dispatchEvent(
        new TouchEvent('touchmove', {
          touches: [
            new Touch({
              target,
              identifier: 0,
              clientX: 20 + rect.left,
              clientY: 20 + rect.top,
            }),
          ],
        })
      )
      target.dispatchEvent(
        new TouchEvent('touchend', {
          touches: [
            new Touch({
              target,
              identifier: 0,
              clientX: 20 + rect.left,
              clientY: 20 + rect.top,
            }),
          ],
        })
      )

      expect(events).to.have.lengthOf(3)
      expect(events[0].detail).to.deep.equal({ x: 0, y: 0 })
      expect(events[1].detail).to.deep.equal({ x: 10 / 100, y: 10 / 100 })
      expect(events[2].detail).to.deep.equal({ x: 20 / 100, y: 20 / 100 })
    })
  })

  it('should not trigger mouse events when attached element has not been clicked', () => {
    const events: TouchPadMoveEvent[] = []
    touchPad.addEventListener('touchpadmove', (event) => {
      events.push(event as TouchPadMoveEvent)
    })

    document.dispatchEvent(new MouseEvent('mousemove', { clientX: 10, clientY: 10 }))
    document.dispatchEvent(new MouseEvent('mousemove', { clientX: 20, clientY: 20 }))
    document.dispatchEvent(new MouseEvent('mouseup', { clientX: 20, clientY: 20 }))

    target.dispatchEvent(
      new MouseEvent('mouseup', {
        clientX: 0,
        clientY: 0,
        bubbles: true,
      })
    )
    target.dispatchEvent(
      new MouseEvent('mousemove', {
        clientX: 10,
        clientY: 10,
        bubbles: true,
      })
    )

    expect(events).to.have.lengthOf(0)
  })

  describe('when attached element is right clicked', () => {
    it('should not emit events', () => {
      const events: TouchPadMoveEvent[] = []
      touchPad.addEventListener('touchpadmove', (event) => {
        events.push(event as TouchPadMoveEvent)
      })

      const rect = target.getBoundingClientRect()
      target.dispatchEvent(
        new MouseEvent('mousedown', {
          clientX: 0 + rect.left,
          clientY: 0 + rect.top,
          button: 2,
          bubbles: true,
        })
      )
      target.dispatchEvent(
        new MouseEvent('mousemove', {
          clientX: 10 + rect.left,
          clientY: 10 + rect.top,
          button: 2,
          bubbles: true,
        })
      )
      target.dispatchEvent(
        new MouseEvent('mousemove', {
          clientX: 20 + rect.left,
          clientY: 20 + rect.top,
          button: 2,
          bubbles: true,
        })
      )
      target.dispatchEvent(
        new MouseEvent('mouseup', {
          clientX: 20 + rect.left,
          clientY: 20 + rect.top,
          button: 2,
          bubbles: true,
        })
      )

      expect(events).to.have.lengthOf(0)
    })
  })

  describe('detach()', () => {
    it('should cause it to stop emitting events', () => {
      const events: TouchPadMoveEvent[] = []
      touchPad.addEventListener('touchpadmove', (event) => {
        events.push(event as TouchPadMoveEvent)
      })

      const rect = target.getBoundingClientRect()
      target.dispatchEvent(
        new MouseEvent('mousedown', {
          clientX: 0 + rect.left,
          clientY: 0 + rect.top,
          bubbles: true,
        })
      )
      target.dispatchEvent(
        new MouseEvent('mousemove', {
          clientX: 10 + rect.left,
          clientY: 10 + rect.top,
          bubbles: true,
        })
      )
      touchPad.disconnect()
      target.dispatchEvent(
        new MouseEvent('mousemove', {
          clientX: 20 + rect.left,
          clientY: 20 + rect.top,
          bubbles: true,
        })
      )
      target.dispatchEvent(
        new MouseEvent('mouseup', {
          clientX: 20 + rect.left,
          clientY: 20 + rect.top,
          bubbles: true,
        })
      )

      expect(events).to.have.lengthOf(2)
      expect(events[0].detail).to.deep.equal({ x: 0, y: 0 })
      expect(events[1].detail).to.deep.equal({ x: 10 / 100, y: 10 / 100 })
    })
  })
})
