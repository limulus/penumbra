const { build } = require('esbuild')

async function bundle() {
  const entryPoints = ['src/www/**/index.ts', 'src/www/**/worker.ts']
  const outdir = 'dist/www/assets/js'

  for (const entryPoint of entryPoints) {
    const format = entryPoint.includes('worker.ts') ? 'iife' : 'esm'

    await build({
      entryPoints: [entryPoint],
      outdir,
      bundle: true,
      format,
      minify: true,
      sourcemap: true,
    })
  }
}

module.exports = { bundle }
