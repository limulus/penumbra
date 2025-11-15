# Penumbra - Ray Tracer Challenge Implementation

## Project Overview

Penumbra is an implementation of [The Ray Tracer Challenge](https://pragprog.com/titles/jbtracer/the-ray-tracer-challenge/) using modern web technologies. It combines TypeScript and Rust/WebAssembly to create a high-performance ray tracer that runs in browsers and Node.js environments.

## Tech Stack

- **TypeScript**: Primary language for high-level ray tracing logic
- **Rust/WebAssembly**: Performance-critical computations (SIMD-optimized)
- **Vitest**: Testing framework for TypeScript code
- **wasm-pack**: Build tool for Rust WebAssembly modules
- **Node.js**: Runtime environment (^20.8.0 required)

## Project Structure

```
penumbra/
├── src/                    # TypeScript source code
│   ├── lib/               # Core ray tracing library
│   │   ├── tuple.ts       # 3D points and vectors
│   │   ├── matrix.ts      # Matrix operations and transformations
│   │   ├── ray.ts         # Ray definitions and operations
│   │   ├── sphere.ts      # Sphere primitives
│   │   ├── intersection.ts # Ray-object intersection handling
│   │   ├── canvas.ts      # Image canvas representation
│   │   └── util/          # Utility functions
│   ├── test/              # Test utilities
│   └── index.ts           # Main entry point
│
├── wasm/                   # Rust/WebAssembly source code
│   ├── src/
│   │   ├── lib.rs         # Library entry point
│   │   ├── tuple.rs       # WASM tuple implementation
│   │   ├── matrix.rs      # WASM matrix operations
│   │   ├── ray.rs         # WASM ray operations
│   │   ├── sphere.rs      # WASM sphere implementation
│   │   ├── intersection.rs # WASM intersection handling
│   │   ├── canvas.rs      # WASM canvas operations
│   │   └── demo/          # Demo implementations
│   └── Cargo.toml         # Rust package configuration
│
├── dist/                   # Build output (generated)
│   ├── cjs/               # CommonJS build
│   ├── esm/               # ES Modules build
│   ├── types/             # TypeScript type definitions
│   └── wasm/              # Compiled WebAssembly modules
│
└── .github/workflows/     # CI/CD configuration

```

## Key Components

### Ray Tracing Primitives

- **Tuples**: Fundamental 3D data structure representing both points (w=1) and vectors (w=0)
- **Matrices**: 4x4 transformation matrices for translations, rotations, scaling, and shearing
- **Rays**: Define light rays with origin and direction
- **Spheres**: Basic geometric primitive for ray intersection
- **Intersections**: Track where rays intersect with objects
- **Canvas**: Pixel buffer for rendering images

### Dual Implementation

The project maintains parallel implementations:
- **TypeScript** (src/lib/): Portable, readable reference implementation
- **Rust** (wasm/src/): High-performance SIMD-optimized WebAssembly version

Both implementations follow the same API contract and are tested independently.

## Development Workflow

### Setup

```bash
npm install
```

This will install dependencies and set up git hooks via husky.

### Building

```bash
# Build everything (TypeScript + WASM)
npm run build

# Build specific targets
npm run build:cjs      # CommonJS build
npm run build:esm      # ES Modules build
npm run build:dts      # TypeScript declarations
npm run build:wasm     # Rust/WebAssembly build
```

### Testing

```bash
# Run all tests
npm test

# Run specific test suites
npm run test:browser   # Vitest browser tests
npm run test:wasm      # Rust tests via wasm-pack

# Watch mode for Rust tests
npm run test:wasm:watch
```

### Linting

```bash
# Run all lints and tests
npm run verify

# Lint TypeScript
npm run lint

# Lint Rust
npm run lint:rust:fmt     # Check formatting
npm run lint:rust:clippy  # Run clippy lints
```

### Development

```bash
# Watch Rust files and rebuild WASM on changes
npm run dev

# Type-check TypeScript without emitting
npm run tscc
```

## Module Exports

The package provides multiple export paths:

- **Main export** (`@limulus/penumbra`): TypeScript implementation
  - CJS: `dist/cjs/index.js`
  - ESM: `dist/esm/index.js`
  - Types: `dist/types/index.d.ts`

- **WASM export** (`@limulus/penumbra/wasm/simd`): WebAssembly SIMD-optimized version
  - Import: `dist/wasm/penumbra-simd.js`
  - Types: `dist/wasm/penumbra-simd.d.ts`

## Code Quality Tools

- **ESLint**: TypeScript linting with `@limulus/eslint-config`
- **Clippy**: Rust linting with strict correctness checks (deny level)
- **Rustfmt**: Rust code formatting
- **Commitlint**: Conventional commit message enforcement
- **Husky**: Git hooks for pre-commit checks
- **Semantic Release**: Automated version management and releases

## Rust Configuration Notes

### Cargo Features

- `default`: Includes `console_error_panic_hook` for better debugging
- `release`: Release-specific optimizations
- `simd`: Enable SIMD optimizations for WebAssembly

### Build Profile

The release profile is optimized for WebAssembly:
- `opt-level = "z"`: Optimize for size
- `lto = true`: Link-time optimization enabled
- `codegen-units = 1`: Single codegen unit for better optimization

### Clippy Configuration

- **Correctness** (deny): Actual bugs are denied
- **Suspicious** (warn): Suspicious patterns trigger warnings
- **Performance** (warn): Performance improvements suggested
- **erasing_op** (allow): Allows `row * 4 + col` patterns for clarity

## Testing Strategy

- **Vitest**: Browser-based testing for TypeScript with coverage via Istanbul
- **wasm-pack test**: Node-based testing for Rust WebAssembly modules
- Both test suites run in CI and are required for merging

## Documentation

Progress and interactive demos are documented at:
- Website: https://limulus.net/penumbra/
- Website repo: https://github.com/limulus/penumbra-www/

## Common Tasks for Claude

### Adding a New Ray Tracing Feature

1. Implement in TypeScript first (src/lib/)
2. Write tests using Vitest (*.spec.ts)
3. Add Rust implementation (wasm/src/)
4. Write Rust tests using #[cfg(test)]
5. Export from src/index.ts if public API
6. Update WASM bindings if needed
7. Run `npm run verify` to ensure all checks pass

### Debugging Issues

- TypeScript errors: Check `npm run tscc`
- Rust errors: Check `cargo clippy --manifest-path wasm/Cargo.toml`
- Test failures: Run `npm test` or specific test commands
- Build failures: Clean with `npm run clean` and rebuild

### Making Changes

- Follow conventional commits format (commitlint enforced)
- Run `npm run verify` before committing
- TypeScript files should maintain type safety
- Rust code must pass clippy at deny level for correctness
