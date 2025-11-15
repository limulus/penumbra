#!/usr/bin/env node

/**
 * WASM Performance Benchmark Suite
 *
 * This script measures the performance of WASM operations by loading the compiled
 * WASM module and using Node.js performance timing APIs to measure execution time.
 *
 * Usage:
 *   npm run bench:wasm
 *   npm run bench:wasm > baseline.txt
 */

import fs from 'node:fs'
import path from 'node:path'
import { performance } from 'node:perf_hooks'
import { fileURLToPath } from 'node:url'

// Get the directory of this file
const __dirname = path.dirname(fileURLToPath(import.meta.url))
const wasmDir = path.join(__dirname, '../dist/wasm')

// Check if WASM has been built
const wasmFile = path.join(wasmDir, 'penumbra-simd.js')
if (!fs.existsSync(wasmFile)) {
  console.error(`Error: WASM not built. Run 'npm run build:wasm' first.`)
  process.exit(1)
}

// Import the WASM module
const { BenchOps } = await import(`file://${wasmFile}`)

// Configuration
const WARMUP_ITERATIONS = 1000
const BENCH_ITERATIONS = 100000
const RUNS = 10

/**
 * Run a benchmark and report statistics
 * @param {string} name - Benchmark name
 * @param {Function} fn - Benchmark function that accepts iteration count
 * @param {number} iterations - Number of iterations to run
 */
async function benchmark(name, fn, iterations = BENCH_ITERATIONS) {
  // Warmup phase
  fn(WARMUP_ITERATIONS)

  const times = []

  // Run benchmark multiple times
  for (let i = 0; i < RUNS; i++) {
    const start = performance.now()
    fn(iterations)
    const end = performance.now()
    times.push(end - start)
  }

  // Calculate statistics
  const avg = times.reduce((a, b) => a + b, 0) / times.length
  const min = Math.min(...times)
  const max = Math.max(...times)
  const perOp = (avg / iterations) * 1000000 // nanoseconds per operation

  // Calculate standard deviation
  const variance =
    times.reduce((sum, time) => sum + Math.pow(time - avg, 2), 0) / times.length
  const stdDev = Math.sqrt(variance)
  const stdDevPercent = ((stdDev / avg) * 100).toFixed(1)

  // Print results
  console.log(`${name}:`)
  console.log(`  Iterations: ${iterations.toLocaleString()}`)
  console.log(`  Avg: ${avg.toFixed(2)}ms (${perOp.toFixed(0)}ns/op)`)
  console.log(`  Min: ${min.toFixed(2)}ms, Max: ${max.toFixed(2)}ms`)
  console.log(`  Std Dev: ${stdDev.toFixed(2)}ms (${stdDevPercent}%)`)
  console.log('')
}

/**
 * Print a section header
 * @param {string} title - Section title
 */
function printHeader(title) {
  console.log(title)
  console.log('='.repeat(title.length))
  console.log('')
}

/**
 * Main benchmark suite
 */
async function main() {
  const bench = new BenchOps()

  console.log('WASM Performance Benchmarks')
  console.log('')
  console.log(`Configuration:`)
  console.log(`  Warmup iterations: ${WARMUP_ITERATIONS.toLocaleString()}`)
  console.log(`  Bench iterations: ${BENCH_ITERATIONS.toLocaleString()}`)
  console.log(`  Runs: ${RUNS}`)
  console.log('')
  console.log('='.repeat(60))
  console.log('')

  // Matrix operations
  printHeader('Matrix Operations')

  await benchmark('Matrix 4x4 Multiplication', (n) => bench.matrix_multiply_bench(n))

  await benchmark('Matrix Chain (4x multiply)', (n) => bench.matrix_chain_multiply_bench(n))

  await benchmark('Matrix × Point', (n) => bench.matrix_point_multiply_bench(n))

  await benchmark('Matrix × Vector', (n) => bench.matrix_vector_multiply_bench(n))

  await benchmark('Matrix Transpose', (n) => bench.matrix_transpose_bench(n))

  await benchmark('Matrix Determinant', (n) => bench.matrix_determinant_bench(n))

  // Tuple operations
  printHeader('Tuple Operations')

  await benchmark('Tuple Addition', (n) => bench.tuple_add_bench(n))

  await benchmark('Tuple Subtraction', (n) => bench.tuple_subtract_bench(n))

  await benchmark('Tuple Scalar Multiply', (n) => bench.tuple_scalar_multiply_bench(n))

  await benchmark('Tuple Dot Product', (n) => bench.tuple_dot_product_bench(n))

  await benchmark('Tuple Cross Product', (n) => bench.tuple_cross_product_bench(n))

  // Cleanup
  bench.free()

  console.log('='.repeat(60))
  console.log('Benchmark complete')
}

main().catch((err) => {
  console.error('Benchmark failed:', err)
  process.exit(1)
})
