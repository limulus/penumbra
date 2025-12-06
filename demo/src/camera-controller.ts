// Camera control constants
const CAMERA_DISTANCE = 8.0
const CAMERA_HEIGHT_AT_CENTER = 1.5
const AZIMUTH_RANGE = Math.PI * 1.5 // 270° horizontal rotation
const ELEVATION_RANGE = Math.PI * 0.6 // ~108° vertical range

// Coordinate types
export interface SphericalCoords {
  azimuth: number
  elevation: number
}

export interface CartesianCoords {
  x: number
  y: number
  z: number
}

/**
 * Convert normalized touchpad coordinates (0-1) to spherical angles
 */
export function touchpadToSpherical(x: number, y: number): SphericalCoords {
  // x controls azimuth (horizontal rotation)
  // Map x: 0->1 to azimuth: -135° to +135° (centered at 0°, which is -Z axis)
  const azimuth = (x - 0.5) * AZIMUTH_RANGE

  // y controls elevation (vertical angle from horizontal plane)
  // Map y: 0->1 to elevation: -54° to +54° (centered at 0°, horizontal)
  const elevation = (0.5 - y) * ELEVATION_RANGE

  return { azimuth, elevation }
}

/**
 * Convert spherical coordinates to Cartesian position
 */
export function sphericalToCartesian(
  azimuth: number,
  elevation: number
): CartesianCoords {
  const x = CAMERA_DISTANCE * Math.cos(elevation) * Math.sin(azimuth)
  const y = CAMERA_HEIGHT_AT_CENTER + CAMERA_DISTANCE * Math.sin(elevation)
  const z = -CAMERA_DISTANCE * Math.cos(elevation) * Math.cos(azimuth)

  return { x, y, z }
}

/**
 * Camera state manager
 */
export class CameraController {
  private azimuth: number = 0
  private elevation: number = 0
  private isDragging: boolean = false

  /**
   * Update camera position from touchpad coordinates
   */
  updatePosition(x: number, y: number): void {
    const spherical = touchpadToSpherical(x, y)
    this.azimuth = spherical.azimuth
    this.elevation = spherical.elevation
  }

  /**
   * Get current camera position in 3D space
   */
  getCameraPosition(): CartesianCoords {
    return sphericalToCartesian(this.azimuth, this.elevation)
  }

  /**
   * Set dragging state
   */
  setDragging(dragging: boolean): void {
    this.isDragging = dragging
  }

  /**
   * Check if currently dragging
   */
  isCurrentlyDragging(): boolean {
    return this.isDragging
  }

  /**
   * Reset camera to default position
   */
  reset(): void {
    this.azimuth = 0
    this.elevation = 0
    this.isDragging = false
  }
}
