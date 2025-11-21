#!/usr/bin/env node

/**
 * WASM SIMD vs Scalar Performance Comparison
 *
 * This script compares the performance of WASM SIMD-optimized operations
 * against their scalar (non-SIMD) equivalents.
 *
 * The `wide` crate automatically falls back to scalar implementations when
 * compiled without the +simd128 target feature, allowing us to measure
 * the real-world performance impact of SIMD optimizations.
 *
 * Usage:
 *   npm run bench:compare
 *   npm run bench:compare > comparison-results.txt
 */

import fs from 'node:fs'
import path from 'node:path'
import { performance } from 'node:perf_hooks'
import { fileURLToPath } from 'node:url'

// Get the directory of this file
const __dirname = path.dirname(fileURLToPath(import.meta.url))
const wasmDir = path.join(__dirname, '../dist/wasm')

// Check if both WASM builds exist
const simdFile = path.join(wasmDir, 'penumbra-simd.js')
const scalarFile = path.join(wasmDir, 'penumbra-scalar.js')

if (!fs.existsSync(simdFile)) {
  console.error(
    `Error: SIMD WASM not built. Run 'npm run build:wasm:simd' first.`,
  )
  process.exit(1)
}

if (!fs.existsSync(scalarFile)) {
  console.error(
    `Error: Scalar WASM not built. Run 'npm run build:wasm:scalar' first.`,
  )
  process.exit(1)
}

// Import both WASM modules
const simdModule = await import(`file://${simdFile}`)
const scalarModule = await import(`file://${scalarFile}`)

// Configuration
const WARMUP_ITERATIONS = 1000
const BENCH_ITERATIONS = 100000
const RUNS = 10

/**
 * Run a benchmark and return statistics
 * @param {Function} fn - Benchmark function that accepts iteration count
 * @param {number} iterations - Number of iterations to run
 * @returns {Object} Statistics object with avg, min, max, stdDev
 */
function runBenchmark(fn, iterations = BENCH_ITERATIONS) {
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

  // Calculate standard deviation
  const variance =
    times.reduce((sum, time) => sum + Math.pow(time - avg, 2), 0) / times.length
  const stdDev = Math.sqrt(variance)

  return { avg, min, max, stdDev, times }
}

/**
 * Compare SIMD vs Scalar performance
 * @param {string} name - Benchmark name
 * @param {Function} simdFn - SIMD version benchmark function
 * @param {Function} scalarFn - Scalar version benchmark function
 * @param {number} iterations - Number of iterations to run
 */
function compareBenchmark(name, simdFn, scalarFn, iterations = BENCH_ITERATIONS) {
  const simdStats = runBenchmark(simdFn, iterations)
  const scalarStats = runBenchmark(scalarFn, iterations)

  const speedup = scalarStats.avg / simdStats.avg
  const speedupPercent = ((speedup - 1) * 100).toFixed(1)

  const simdPerOp = (simdStats.avg / iterations) * 1000000 // nanoseconds per operation
  const scalarPerOp = (scalarStats.avg / iterations) * 1000000

  console.log(`${name}:`)
  console.log(
    `  SIMD:   ${simdStats.avg.toFixed(2)}ms (${simdPerOp.toFixed(0)}ns/op) ±${((simdStats.stdDev / simdStats.avg) * 100).toFixed(1)}%`,
  )
  console.log(
    `  Scalar: ${scalarStats.avg.toFixed(2)}ms (${scalarPerOp.toFixed(0)}ns/op) ±${((scalarStats.stdDev / scalarStats.avg) * 100).toFixed(1)}%`,
  )

  if (speedup > 1.05) {
    console.log(`  ⚡ Speedup: ${speedup.toFixed(2)}x (${speedupPercent}% faster)`)
  } else if (speedup < 0.95) {
    console.log(
      `  ⚠️  Slowdown: ${(1 / speedup).toFixed(2)}x (${Math.abs(speedupPercent).toFixed(1)}% slower)`,
    )
  } else {
    console.log(`  ≈  Similar performance (${speedupPercent}% difference)`)
  }
  console.log('')

  return { name, simdStats, scalarStats, speedup }
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
 * Main comparison suite
 */
async function main() {
  const simdBench = new simdModule.BenchOps()
  const scalarBench = new scalarModule.BenchOps()

  const results = []

  console.log('WASM SIMD vs Scalar Performance Comparison')
  console.log('')
  console.log(`Configuration:`)
  console.log(`  Warmup iterations: ${WARMUP_ITERATIONS.toLocaleString()}`)
  console.log(`  Bench iterations: ${BENCH_ITERATIONS.toLocaleString()}`)
  console.log(`  Runs per benchmark: ${RUNS}`)
  console.log('')

  // Get WASM file sizes
  const simdWasmPath = path.join(wasmDir, 'penumbra-simd_bg.wasm')
  const scalarWasmPath = path.join(wasmDir, 'penumbra-scalar_bg.wasm')
  const simdSize = fs.statSync(simdWasmPath).size
  const scalarSize = fs.statSync(scalarWasmPath).size

  console.log(`WASM Binary Sizes:`)
  console.log(`  SIMD:   ${(simdSize / 1024).toFixed(2)} KB`)
  console.log(`  Scalar: ${(scalarSize / 1024).toFixed(2)} KB`)
  console.log(
    `  Size diff: ${((simdSize - scalarSize) / 1024).toFixed(2)} KB (${(((simdSize - scalarSize) / scalarSize) * 100).toFixed(1)}%)`,
  )
  console.log('')
  console.log('='.repeat(60))
  console.log('')

  // Matrix operations
  printHeader('Matrix Operations')

  results.push(
    compareBenchmark(
      'Matrix 4x4 Multiplication',
      (n) => simdBench.matrix_multiply_bench(n),
      (n) => scalarBench.matrix_multiply_bench(n),
    ),
  )

  results.push(
    compareBenchmark(
      'Matrix Chain (4x multiply)',
      (n) => simdBench.matrix_chain_multiply_bench(n),
      (n) => scalarBench.matrix_chain_multiply_bench(n),
    ),
  )

  results.push(
    compareBenchmark(
      'Matrix × Point',
      (n) => simdBench.matrix_point_multiply_bench(n),
      (n) => scalarBench.matrix_point_multiply_bench(n),
    ),
  )

  results.push(
    compareBenchmark(
      'Matrix × Vector',
      (n) => simdBench.matrix_vector_multiply_bench(n),
      (n) => scalarBench.matrix_vector_multiply_bench(n),
    ),
  )

  results.push(
    compareBenchmark(
      'Matrix Transpose',
      (n) => simdBench.matrix_transpose_bench(n),
      (n) => scalarBench.matrix_transpose_bench(n),
    ),
  )

  results.push(
    compareBenchmark(
      'Matrix Determinant',
      (n) => simdBench.matrix_determinant_bench(n),
      (n) => scalarBench.matrix_determinant_bench(n),
    ),
  )

  // Tuple operations
  printHeader('Tuple Operations')

  results.push(
    compareBenchmark(
      'Tuple Addition',
      (n) => simdBench.tuple_add_bench(n),
      (n) => scalarBench.tuple_add_bench(n),
    ),
  )

  results.push(
    compareBenchmark(
      'Tuple Subtraction',
      (n) => simdBench.tuple_subtract_bench(n),
      (n) => scalarBench.tuple_subtract_bench(n),
    ),
  )

  results.push(
    compareBenchmark(
      'Tuple Scalar Multiply',
      (n) => simdBench.tuple_scalar_multiply_bench(n),
      (n) => scalarBench.tuple_scalar_multiply_bench(n),
    ),
  )

  results.push(
    compareBenchmark(
      'Tuple Dot Product',
      (n) => simdBench.tuple_dot_product_bench(n),
      (n) => scalarBench.tuple_dot_product_bench(n),
    ),
  )

  results.push(
    compareBenchmark(
      'Tuple Cross Product',
      (n) => simdBench.tuple_cross_product_bench(n),
      (n) => scalarBench.tuple_cross_product_bench(n),
    ),
  )

  // Summary
  printHeader('Summary')

  const avgSpeedup =
    results.reduce((sum, r) => sum + r.speedup, 0) / results.length
  const maxSpeedup = Math.max(...results.map((r) => r.speedup))
  const minSpeedup = Math.min(...results.map((r) => r.speedup))

  console.log(`Average speedup: ${avgSpeedup.toFixed(2)}x`)
  console.log(`Best speedup: ${maxSpeedup.toFixed(2)}x (${results.find((r) => r.speedup === maxSpeedup).name})`)
  console.log(`Worst speedup: ${minSpeedup.toFixed(2)}x (${results.find((r) => r.speedup === minSpeedup).name})`)
  console.log('')

  const fasterCount = results.filter((r) => r.speedup > 1.05).length
  const slowerCount = results.filter((r) => r.speedup < 0.95).length
  const similarCount = results.length - fasterCount - slowerCount

  console.log(`Operations faster with SIMD: ${fasterCount}/${results.length}`)
  console.log(`Operations slower with SIMD: ${slowerCount}/${results.length}`)
  console.log(`Operations with similar perf: ${similarCount}/${results.length}`)

  // Cleanup
  simdBench.free()
  scalarBench.free()

  console.log('')
  console.log('='.repeat(60))
  console.log('Comparison complete')
}

main().catch((err) => {
  console.error('Comparison failed:', err)
  process.exit(1)
})
