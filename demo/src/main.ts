import './style.css'
import 'touch-pad/define'
import { Scene, Transform } from '../../dist/index.js'
import { renderFromBuffer } from '../../dist/wasm/penumbra-simd.js'
import {
  CameraController,
  type CartesianCoords,
} from './camera-controller.js'

// Constants
const PREVIEW_SCALE = 0.25
const DEBOUNCE_DELAY_MS = 300

// DOM elements
const renderForm = document.getElementById('render-form') as HTMLFormElement
const touchPad = document.querySelector('touch-pad')!
const canvas = document.getElementById('canvas') as HTMLCanvasElement
const ctx = canvas.getContext('2d')!
const widthInput = document.getElementById('width') as HTMLInputElement
const heightInput = document.getElementById('height') as HTMLInputElement
const fovInput = document.getElementById('fov') as HTMLInputElement
const fovValue = document.getElementById('fov-value')!
const renderTime = document.getElementById('render-time')!

// Camera controller
const cameraController = new CameraController()

// Single scene (shared by preview and full-res)
let scene: Scene

// Render worker
const renderWorker = new Worker(
  new URL('./render-worker.ts', import.meta.url),
  { type: 'module' }
)

// State
let debounceTimer: number | null = null
let workerReady = false

// Initialize scene
function initScene() {
  scene = new Scene()

  // Floor
  scene.addSphere({
    transform: new Transform().scale(10, 0.01, 10),
    color: [1.0, 0.9, 0.9],
    material: { ambient: 0.1, diffuse: 0.9, specular: 0.0, shininess: 200 },
  })

  // Middle sphere
  scene.addSphere({
    transform: new Transform().translate(-0.5, 1, 0.5),
    color: [0.1, 1.0, 0.5],
    material: { ambient: 0.1, diffuse: 0.7, specular: 0.3, shininess: 200 },
  })

  // Right sphere
  scene.addSphere({
    transform: new Transform().scale(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5),
    color: [0.5, 1.0, 0.1],
    material: { ambient: 0.1, diffuse: 0.7, specular: 0.3, shininess: 200 },
  })

  // Left sphere
  scene.addSphere({
    transform: new Transform().scale(0.33, 0.33, 0.33).translate(-1.5, 0.33, -0.75),
    color: [1.0, 0.8, 0.1],
    material: { ambient: 0.1, diffuse: 0.7, specular: 0.3, shininess: 200 },
  })

  // Light
  scene.setLight({
    position: [-10, 10, -10],
    intensity: [1, 1, 1],
  })

  // Send buffer to worker
  renderWorker.postMessage({ type: 'init', buffer: scene.getBuffer() })
}

function updateCamera(cameraPos: CartesianCoords) {
  scene.lookAt({
    from: [cameraPos.x, cameraPos.y, cameraPos.z],
    to: [0, 0, 0],
    up: [0, 1, 0],
  })
}

function getFOV(): number {
  return (parseInt(fovInput.value) * Math.PI) / 180
}

function getFullDimensions() {
  return {
    width: parseInt(widthInput.value),
    height: parseInt(heightInput.value),
  }
}

function getPreviewDimensions() {
  const full = getFullDimensions()
  return {
    width: Math.floor(full.width * PREVIEW_SCALE),
    height: Math.floor(full.height * PREVIEW_SCALE),
  }
}

// Preview render (main thread) - uses same buffer with different dimensions
async function doPreviewRender(cameraPos: CartesianCoords) {
  updateCamera(cameraPos)

  const { width, height } = getPreviewDimensions()
  const fov = getFOV()
  const view = new Uint8Array(scene.getBuffer())
  const startTime = performance.now()

  const imageData = renderFromBuffer(view, width, height, fov)

  const duration = performance.now() - startTime

  // Scale up to canvas
  const bitmap = await createImageBitmap(imageData)
  ctx.drawImage(bitmap, 0, 0, canvas.width, canvas.height)
  bitmap.close()

  renderTime.textContent = `Preview: ${duration.toFixed(2)}ms`
}

// Full-res render (worker) - uses same buffer with full dimensions
function requestFullRender(cameraPos: CartesianCoords) {
  if (!workerReady) return

  updateCamera(cameraPos)

  const { width, height } = getFullDimensions()
  const fov = getFOV()

  // Only resize canvas if dimensions changed (resizing clears the canvas)
  if (canvas.width !== width || canvas.height !== height) {
    canvas.width = width
    canvas.height = height
  }

  renderTime.textContent = 'Rendering…'
  renderWorker.postMessage({ type: 'render', width, height, fov })
}

// Worker message handler
renderWorker.onmessage = (e: MessageEvent) => {
  if (e.data.type === 'ready') {
    workerReady = true
    // Initial render
    const cameraPos = cameraController.getCameraPosition()
    requestFullRender(cameraPos)
  }

  if (e.data.type === 'complete') {
    const { imageData, duration } = e.data
    ctx.putImageData(imageData, 0, 0)
    renderTime.textContent = `Rendered in ${duration.toFixed(2)}ms`
  }

  if (e.data.type === 'error') {
    renderTime.textContent = `Error: ${e.data.message}`
    console.error('Render worker error:', e.data.message)
  }
}

// Schedule full-res render with debouncing
function scheduleFullResRender(cameraPos: CartesianCoords) {
  if (debounceTimer !== null) {
    clearTimeout(debounceTimer)
  }

  debounceTimer = setTimeout(() => {
    requestFullRender(cameraPos)
    debounceTimer = null
  }, DEBOUNCE_DELAY_MS)
}

// Event handlers
touchPad.addEventListener('touchpadmove', (e: Event) => {
  const customEvent = e as CustomEvent
  const { x, y } = customEvent.detail

  cameraController.updatePosition(x, y)
  const cameraPos = cameraController.getCameraPosition()

  doPreviewRender(cameraPos)
  scheduleFullResRender(cameraPos)
})

fovInput.addEventListener('input', () => {
  fovValue.textContent = `${fovInput.value}°`

  const cameraPos = cameraController.getCameraPosition()
  doPreviewRender(cameraPos)
  scheduleFullResRender(cameraPos)
})

renderForm.addEventListener('submit', (e) => {
  e.preventDefault()
  const cameraPos = cameraController.getCameraPosition()
  requestFullRender(cameraPos)
})

// Initialize
const { width, height } = getFullDimensions()
canvas.width = width
canvas.height = height

initScene()
