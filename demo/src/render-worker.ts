let renderFromBuffer: (
  buffer: Uint8Array,
  width: number,
  height: number,
  fov: number
) => ImageData
let sceneBuffer: SharedArrayBuffer | null = null
let wasmReady = false
const pendingMessages: MessageEvent[] = []

// Set up message handler immediately to queue messages
self.onmessage = (e: MessageEvent) => {
  if (!wasmReady) {
    pendingMessages.push(e)
    return
  }

  handleMessage(e)
}

function handleMessage(e: MessageEvent) {
  if (e.data.type === 'init') {
    sceneBuffer = e.data.buffer
    self.postMessage({ type: 'ready' })
  }

  if (e.data.type === 'render') {
    if (!sceneBuffer) {
      self.postMessage({ type: 'error', message: 'Buffer not initialized' })
      return
    }

    const { width, height, fov } = e.data
    const view = new Uint8Array(sceneBuffer)
    const startTime = performance.now()

    try {
      const wasmImageData = renderFromBuffer(view, width, height, fov)
      const duration = performance.now() - startTime

      // Copy ImageData since WASM memory can't be transferred
      const imageData = new ImageData(
        new Uint8ClampedArray(wasmImageData.data),
        wasmImageData.width,
        wasmImageData.height
      )

      self.postMessage({ type: 'complete', imageData, duration }, {
        transfer: [imageData.data.buffer],
      })
    } catch (error) {
      self.postMessage({ type: 'error', message: String(error) })
    }
  }
}

// Initialize WASM
import('../../dist/wasm/penumbra-simd.js')
  .then((module) => {
    renderFromBuffer = module.renderFromBuffer
    wasmReady = true

    // Process any queued messages
    for (const msg of pendingMessages) {
      handleMessage(msg)
    }
    pendingMessages.length = 0
  })
  .catch((error) => {
    console.error('[Worker] Failed to load WASM:', error)
  })
