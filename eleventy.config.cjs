const pluginWebc = require('@11ty/eleventy-plugin-webc')

/** @param {import("@11ty/eleventy").UserConfig} eleventyConfig */
module.exports = function (eleventyConfig) {
  const input = 'www'
  const output = 'dist/www'
  const layouts = '_includes/layouts'
  const components = `${input}/_include/components`

  eleventyConfig.addPlugin(pluginWebc, { components })

  return { dir: { input, output, layouts }, pathPrefix: '/penumbra/' }
}
