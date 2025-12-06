import {
  getSceneBufferSize,
  writeCameraToBuffer,
  writeLightToBuffer,
  initializeSphereInBuffer,
  writeSphereColorToBuffer,
  writeSphereMaterialToBuffer,
  writeSphereTransformToBuffer,
  Transform,
} from '../dist/wasm/penumbra-simd.js'

const MAX_SPHERES = 256

interface SphereConfig {
  transform?: Transform
  color?: [number, number, number]
  material?: { ambient: number; diffuse: number; specular: number; shininess: number }
}

export class Scene {
  private buffer: SharedArrayBuffer
  private view: Uint8Array
  private sphereCount = 0

  constructor() {
    const totalSize = getSceneBufferSize(MAX_SPHERES)
    this.buffer = new SharedArrayBuffer(totalSize)
    this.view = new Uint8Array(this.buffer)
  }

  lookAt(params: {
    from: [number, number, number]
    to: [number, number, number]
    up: [number, number, number]
  }) {
    const [fromX, fromY, fromZ] = params.from
    const [toX, toY, toZ] = params.to
    const [upX, upY, upZ] = params.up
    writeCameraToBuffer(this.view, fromX, fromY, fromZ, toX, toY, toZ, upX, upY, upZ)
  }

  setLight(params: { position: [number, number, number]; intensity: [number, number, number] }) {
    const [posX, posY, posZ] = params.position
    const [intR, intG, intB] = params.intensity
    writeLightToBuffer(this.view, posX, posY, posZ, intR, intG, intB)
  }

  addSphere(config?: SphereConfig): number {
    const id = this.sphereCount++
    initializeSphereInBuffer(this.view, id)

    if (config?.transform) {
      writeSphereTransformToBuffer(config.transform, this.view, id)
    }
    if (config?.color) {
      const [r, g, b] = config.color
      writeSphereColorToBuffer(this.view, id, r, g, b)
    }
    if (config?.material) {
      const { ambient, diffuse, specular, shininess } = config.material
      writeSphereMaterialToBuffer(this.view, id, ambient, diffuse, specular, shininess)
    }

    return id
  }

  setSphereTransform(id: number, transform: Transform) {
    writeSphereTransformToBuffer(transform, this.view, id)
  }

  setSphereColor(id: number, color: [number, number, number]) {
    const [r, g, b] = color
    writeSphereColorToBuffer(this.view, id, r, g, b)
  }

  setSphereMaterial(
    id: number,
    material: { ambient: number; diffuse: number; specular: number; shininess: number }
  ) {
    writeSphereMaterialToBuffer(
      this.view,
      id,
      material.ambient,
      material.diffuse,
      material.specular,
      material.shininess
    )
  }

  getBuffer(): SharedArrayBuffer {
    return this.buffer
  }
}
