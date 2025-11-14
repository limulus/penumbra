# @limulus/penumbra

A [Ray Tracer Challenge](https://pragprog.com/titles/jbtracer/the-ray-tracer-challenge/)
implementation using web technologies

## Development Journal and Demos

I’m documenting my progress and creating interactive demos as I go on the [project’s
website]. The site code is hosted in a [separate repository].

[project’s website]: https://limulus.net/penumbra/
[separate repository]: https://github.com/limulus/penumbra-www/

## Local Development

### Prerequisites

- [Node.js](https://nodejs.org/) ^20.8.0
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
