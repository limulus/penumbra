/// <reference lib="webworker" />

!(async () => {
  if (self.requestAnimationFrame === undefined) {
    self.global = self
    const raf = await import('raf')
    raf.polyfill(self)
  }
})()
