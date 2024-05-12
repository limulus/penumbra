import { coverageConfigDefaults, defineConfig } from 'vitest/config'

export default defineConfig({
  test: {
    browser: {
      name: 'chrome',
      enabled: true,
      headless: true,
    },
    coverage: {
      enabled: true,
      exclude: ['dist/**/*', 'wasm/**/*', ...coverageConfigDefaults.exclude],
      provider: 'istanbul',
    },
  },
})
