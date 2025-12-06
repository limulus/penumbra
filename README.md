# @limulus/penumbra

A [Ray Tracer Challenge](https://pragprog.com/titles/jbtracer/the-ray-tracer-challenge/)
implementation using web technologies

## Requirements

This library uses `SharedArrayBuffer` for high-performance, zero-copy buffer sharing between threads. In browser environments, this requires [cross-origin isolation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/SharedArrayBuffer#security_requirements). Serve your site with these headers:

```
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

Node.js environments have no additional requirements.

## Usage

The `Scene` class provides an ergonomic API for building ray-traced scenes:

```typescript
import { Scene, Transform, renderFromBuffer } from '@limulus/penumbra'

// Create a scene
const scene = new Scene()

// Add spheres with optional configuration
scene.addSphere({
  transform: new Transform().scale(10, 0.01, 10),
  color: [1.0, 0.9, 0.9],
  material: { ambient: 0.1, diffuse: 0.9, specular: 0.0, shininess: 200 },
})

// Or configure spheres incrementally
const sphereId = scene.addSphere()
scene.setSphereTransform(sphereId, new Transform().translate(0, 1, 0))
scene.setSphereColor(sphereId, [0.1, 1.0, 0.5])
scene.setSphereMaterial(sphereId, {
  ambient: 0.1,
  diffuse: 0.7,
  specular: 0.3,
  shininess: 200,
})

// Set up lighting
scene.setLight({
  position: [-10, 10, -10],
  intensity: [1, 1, 1],
})

// Set up camera
scene.lookAt({
  from: [0, 1.5, -5],
  to: [0, 1, 0],
  up: [0, 1, 0],
})

// Render the scene
const width = 800
const height = 600
const fov = Math.PI / 3 // 60 degrees in radians
const imageData = renderFromBuffer(
  new Uint8Array(scene.getBuffer()),
  width,
  height,
  fov
)

// Display in a canvas
const canvas = document.getElementById('canvas') as HTMLCanvasElement
const ctx = canvas.getContext('2d')!
canvas.width = width
canvas.height = height
ctx.putImageData(imageData, 0, 0)
```

The buffer can be shared with Web Workers for non-blocking rendering:

```typescript
// Main thread
const worker = new Worker('./render-worker.js', { type: 'module' })
worker.postMessage({ buffer: scene.getBuffer(), width: 800, height: 600, fov: Math.PI / 3 })

// render-worker.js
import { renderFromBuffer } from '@limulus/penumbra/wasm/simd'

self.onmessage = (e) => {
  const { buffer, width, height, fov } = e.data
  const imageData = renderFromBuffer(new Uint8Array(buffer), width, height, fov)
  self.postMessage({ imageData })
}
```

## Development Journal and Demos

I’m documenting my progress and creating interactive demos as I go on the [project’s
website]. The site code is hosted in a [separate repository].

[project’s website]: https://limulus.net/penumbra/
[separate repository]: https://github.com/limulus/penumbra-www/

## Local Development

### Prerequisites

- [Node.js](https://nodejs.org/) >=22
- [Rust](https://www.rust-lang.org/) (stable toolchain - managed via rust-toolchain.toml)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) - Install via:
  ```sh
  cargo install wasm-pack
  ```

### Installation

```sh
npm install
```

### Running Tests

Single run:

```sh
npm test
```

Watch mode for Rust tests:

```sh
npm run test:wasm:watch
```
