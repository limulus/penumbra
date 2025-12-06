# Penumbra - Ray Tracer Challenge Implementation

## Project Overview

Penumbra is an implementation of [The Ray Tracer Challenge](https://pragprog.com/titles/jbtracer/the-ray-tracer-challenge/) using Rust and WebAssembly. It provides a high-performance, SIMD-optimized ray tracer that runs in browsers and Node.js environments.

## Tech Stack

- **Rust**: Primary language for ray tracing implementation
- **TypeScript**: High-level API wrapper (Scene class)
- **WebAssembly**: Compilation target for browser/Node.js execution
- **wasm-pack**: Build tool for Rust WebAssembly modules
- **Node.js**: Runtime environment for npm scripts and benchmarks

## Project Structure

```
penumbra/
├── src/                    # Source code
│   ├── index.ts           # TypeScript main export
│   ├── scene.ts           # TypeScript Scene class (high-level API)
│   ├── lib.rs             # Rust library entry point
│   ├── tuple.rs           # 3D points and vectors
│   ├── matrix.rs          # Matrix operations and transformations
│   ├── ray.rs             # Ray definitions and operations
│   ├── sphere.rs          # Sphere primitives
│   ├── intersection.rs    # Ray-object intersection handling
│   ├── canvas.rs          # Image canvas representation
│   ├── material.rs        # Material properties
│   ├── light.rs           # Light sources
│   ├── scene.rs           # Scene buffer management (Rust)
│   └── demo/              # Demo implementations
│
├── benchmarks/            # Performance benchmark scripts
│
├── dist/                  # Build output (generated)
│   ├── wasm/             # Compiled WebAssembly modules
│   ├── index.js          # Compiled TypeScript main export
│   ├── index.d.ts        # TypeScript type definitions
│   ├── scene.js          # Compiled Scene class
│   └── scene.d.ts        # Scene type definitions
│
├── Cargo.toml            # Rust package configuration
├── Cargo.lock            # Rust dependency lock file
├── tsconfig.json         # TypeScript configuration
├── rustfmt.toml          # Rust formatting configuration
└── .github/workflows/    # CI/CD configuration
```

## Key Components

### High-Level API

- **Scene**: TypeScript class providing ergonomic API for building ray-traced scenes
  - Uses `SharedArrayBuffer` for zero-copy buffer sharing with Web Workers
  - Manages camera, lights, and spheres through a simple object-based interface
  - 3D points and RGB colors use array syntax: `[x, y, z]` or `[r, g, b]`
- **Transform**: Fluent API for creating transformation matrices (scale, translate, rotate, shear)
- **renderFromBuffer**: WASM function that renders a scene from its buffer representation

### Ray Tracing Primitives (Rust)

- **Tuples**: Fundamental 3D data structure representing both points (w=1) and vectors (w=0)
- **Matrices**: 4x4 transformation matrices for translations, rotations, scaling, and shearing
- **Rays**: Define light rays with origin and direction
- **Spheres**: Basic geometric primitive for ray intersection
- **Intersections**: Track where rays intersect with objects
- **Canvas**: Pixel buffer for rendering images
- **Materials**: Surface properties for lighting calculations
- **Lights**: Point light sources for scene illumination

## Development Workflow

### Prerequisites

Before starting development, ensure you have the following installed:

- **Node.js** - [Download](https://nodejs.org/)
- **Rust** (stable toolchain) - Managed via rust-toolchain.toml
  - Install: [https://rustup.rs/](https://rustup.rs/)
- **wasm-pack** - Build tool for Rust WebAssembly
  - Install via cargo:
    ```bash
    cargo install wasm-pack
    ```
  - Or see [installation guide](https://rustwasm.github.io/wasm-pack/installer/)

### Setup

```bash
npm install
```

This will install dependencies and set up git hooks via husky.

### Building

```bash
# Build everything (WASM + TypeScript)
npm run build

# Build specific targets
npm run build:wasm         # Both WASM variants (SIMD + scalar)
npm run build:wasm:simd    # SIMD-optimized WebAssembly
npm run build:wasm:scalar  # Scalar (non-SIMD) WebAssembly
npm run build:ts           # TypeScript compilation only
```

### Testing

```bash
# Run all tests
npm test

# Run specific test suites
npm run test:wasm:simd    # SIMD tests
npm run test:wasm:scalar  # Scalar tests

# Watch mode for tests
npm run test:wasm:watch
```

### Linting

```bash
# Run all lints and tests
npm run verify

# Run individual lints
npm run lint:rust:fmt     # Check formatting
npm run lint:rust:clippy  # Run clippy lints
```

### Development

```bash
# Watch Rust files and rebuild WASM on changes
npm run dev
```

### Benchmarks

```bash
# Run WASM performance benchmarks
npm run bench:wasm

# Compare SIMD vs scalar performance
npm run bench:compare
```

## Module Exports

The package provides multiple export paths:

- **Main export** (`@limulus/penumbra`): High-level API for most use cases
  - Exports: `Scene`, `Transform`, `renderFromBuffer`
  - Import: `dist/index.js`
  - Types: `dist/index.d.ts`
  - Example: `import { Scene, Transform, renderFromBuffer } from '@limulus/penumbra'`

- **SIMD export** (`@limulus/penumbra/wasm/simd`): Direct WASM access (optimized for Web Workers)
  - Exports: `Transform`, `renderFromBuffer`, and low-level buffer functions
  - Import: `dist/wasm/penumbra-simd.js`
  - Types: `dist/wasm/penumbra-simd.d.ts`
  - Example: `import { renderFromBuffer } from '@limulus/penumbra/wasm/simd'`

- **Scalar export** (`@limulus/penumbra/wasm/scalar`): Non-SIMD WebAssembly version
  - Same exports as SIMD, but without SIMD optimizations
  - Import: `dist/wasm/penumbra-scalar.js`
  - Types: `dist/wasm/penumbra-scalar.d.ts`

## Code Quality Tools

- **TypeScript**: Strict mode enabled with full type checking
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

Global lints configured in Cargo.toml with priority = -1 to allow individual overrides:

- **Correctness** (deny): Actual bugs are denied
- **Suspicious** (warn): Suspicious patterns trigger warnings
- **Performance** (warn): Performance improvements suggested

Individual functions use explicit `#[allow]` attributes for intentional patterns:
- `too_many_arguments`: Matrix constructors intentionally take many parameters for clarity
- `identity_op` and `erasing_op`: Used in matrix indexing to show the `row*cols+col` pattern (e.g., `0*4+0`) for readability

## Testing Strategy

- **wasm-pack test**: Node-based testing for Rust WebAssembly modules
- Both SIMD and scalar test suites run in CI and are required for merging

## Documentation

Progress and interactive demos are documented at:
- Website: https://limulus.net/penumbra/
- Website repo: https://github.com/limulus/penumbra-www/

## Common Tasks for Claude

### Working with the Scene API

The Scene class (TypeScript) is the primary public API. When adding features:

1. **For user-facing features**: Add methods to `src/scene.ts`
   - Use array syntax for 3D points: `[x, y, z]`
   - Use array syntax for RGB colors: `[r, g, b]`
   - Keep object-based parameters for other options (e.g., `material: { ambient, diffuse, ... }`)
2. **For buffer manipulation**: Add Rust functions to `src/scene.rs`
   - Export with `#[wasm_bindgen]` for use by TypeScript
3. **Build and verify**: Run `npm run build` then `npm run verify`

### Adding a New Ray Tracing Feature

1. Implement in Rust (src/)
2. Write tests using #[cfg(test)]
3. Export from src/lib.rs if public API
4. Update WASM bindings if needed (wasm-bindgen)
5. If user-facing, add to Scene class (src/scene.ts)
6. Run `npm run verify` to ensure all checks pass

### Debugging Issues

- TypeScript errors: Check `npm run build:ts` or use IDE diagnostics
- Rust errors: Check `cargo clippy --target wasm32-unknown-unknown`
- Format issues: Check `cargo fmt --check`
- Test failures: Run `npm test`
- Build failures: Clean with `npm run clean` and rebuild

### Git Commit Message Guidelines

This project uses **semantic-release** for automated versioning and releases. Commit message prefixes determine whether a new version is published:

#### Prefixes that TRIGGER releases:

- **feat:** - New features or functionality (minor version bump)
  - Use ONLY for changes to production code that add new capabilities
  - Example: `feat: add color support to tuples`

- **fix:** - Bug fixes in production code (patch version bump)
  - Use ONLY for changes that fix bugs in production code
  - Example: `fix: correct matrix multiplication order`

#### Prefixes that DO NOT trigger releases:

- **docs:** - Documentation changes only
  - Example: `docs: update README with installation steps`

- **chore:** - Maintenance tasks, dependency updates, build config
  - Example: `chore: update wasm-pack to 0.12.0`

- **test:** - Adding or modifying tests only
  - Example: `test: add coverage for edge cases in sphere intersections`

- **refactor:** - Code changes that neither fix bugs nor add features
  - Example: `refactor: simplify matrix inverse calculation`

- **ci:** - CI/CD configuration changes
  - Example: `ci: add rust caching to GitHub Actions`

- **perf:** - Performance improvements that don't change functionality
  - Example: `perf: optimize tuple normalization`

- **style:** - Code style/formatting changes
  - Example: `style: fix clippy warnings`

#### Important Guidelines:

- **Be conservative with feat/fix**: Only use these when you are changing production code behavior or adding new production features
- **Use chore for tooling**: Changes to tests, docs, build scripts, CI, etc. should use appropriate non-releasing prefixes
- **Commitlint enforces format**: All commits must follow conventional commit format
- **Run verification**: Always run `npm run verify` before committing

#### Examples:

```bash
# Production code changes (WILL release):
git commit -m "feat: implement ray-sphere intersection"
git commit -m "fix: handle divide-by-zero in matrix inverse"

# Non-production changes (will NOT release):
git commit -m "docs: add CLAUDE.md with project documentation"
git commit -m "test: add test cases for transformation matrices"
git commit -m "chore: update dependencies"
git commit -m "refactor: extract common matrix operations"
```

### Making Changes

- Follow conventional commits format (commitlint enforced)
- Run `npm run verify` before committing
- Rust code must pass clippy at deny level for correctness
- Use appropriate commit prefixes (see Git Commit Message Guidelines above)
