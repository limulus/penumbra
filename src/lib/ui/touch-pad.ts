/**
 * Event handler for touchpad-like input. The user can click (or touch) and drag on the
 * given element to emit events with the coordinates of the pointer relative to the element
 * and its size.
 */
export class TouchPad extends EventTarget implements EventListenerObject {
  private readonly element: HTMLElement

  constructor(element: HTMLElement) {
    super()
    this.element = element
    this.element.addEventListener('mousedown', this)
    this.element.addEventListener('touchstart', this)
  }

  handleEvent(event: MouseEvent | TouchEvent) {
    if (event.target === this.element) {
      if (event.type === 'mousedown') {
        this.element.ownerDocument!.addEventListener('mousemove', this)
        this.element.ownerDocument!.addEventListener('mouseup', this)
      } else if (event.type === 'touchstart') {
        this.element.addEventListener('touchmove', this)
        this.element.addEventListener('touchend', this)
      }
    }

    switch (event.type) {
      case 'mousedown':
      case 'mousemove':
      case 'touchstart':
      case 'touchmove':
        this.#emitEvent(event)
        break
      case 'mouseup':
      case 'touchend':
        this.#emitEvent(event)
        this.element.ownerDocument!.removeEventListener('mousemove', this)
        this.element.ownerDocument!.removeEventListener('mouseup', this)
        this.element.removeEventListener('touchmove', this)
        this.element.removeEventListener('touchend', this)
        break
      default:
        throw new Error(`Unhandled event type: ${event.type}`)
    }
  }

  #emitEvent(event: MouseEvent | TouchEvent) {
    event.preventDefault()

    let coords: { clientX: number; clientY: number }
    if (event instanceof MouseEvent) {
      coords = event
    } else if (self.TouchEvent && event instanceof self.TouchEvent) {
      coords = event.touches[0]
    } else {
      throw new TypeError('Expected MouseEvent or TouchEvent')
    }

    const rect = this.element.getBoundingClientRect()
    const x = (coords.clientX - rect.left) / rect.width
    const y = (coords.clientY - rect.top) / rect.height
    this.dispatchEvent(new TouchPadMoveEvent(x, y))
  }

  /**
   * Disconnect this touchpad from the element it was attached to.
   */
  disconnect() {
    for (const event of [
      'mousedown',
      'mousemove',
      'mouseup',
      'touchstart',
      'touchmove',
      'touchend',
    ]) {
      this.element.removeEventListener(event, this)
      this.element.ownerDocument!.removeEventListener(event, this)
    }
  }
}

export class TouchPadMoveEvent extends CustomEvent<{ x: number; y: number }> {
  constructor(x: number, y: number) {
    super('touchpadmove', { detail: { x, y } })
  }
}
