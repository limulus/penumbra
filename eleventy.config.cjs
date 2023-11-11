const { EleventyHtmlBasePlugin } = require('@11ty/eleventy')
const pluginRss = require('@11ty/eleventy-plugin-rss')
const pluginWebc = require('@11ty/eleventy-plugin-webc')
const markdownIt = require('markdown-it')
const markdownItFootnote = require('markdown-it-footnote')

/** @param {import("@11ty/eleventy").UserConfig} eleventyConfig */
module.exports = function (eleventyConfig) {
  const input = 'www'
  const output = 'dist/www'
  const layouts = '_includes/layouts'
  const components = `${input}/_includes/components/**/*.webc`

  eleventyConfig.addPlugin(EleventyHtmlBasePlugin)
  eleventyConfig.addPlugin(pluginRss)
  eleventyConfig.addPlugin(pluginWebc, { components })

  eleventyConfig.amendLibrary('md', (mdLib) => mdLib.use(markdownItFootnote))

  // Markdownify filter
  const md = markdownIt({ html: true, linkify: true })
  eleventyConfig.addFilter('markdownify', (mdString) => md.render(mdString))

  eleventyConfig.setServerOptions({
    domDiff: false,
    watch: ['dist/www/tests/mocha/*'],
  })

  return { dir: { input, output, layouts }, pathPrefix: '/penumbra/' }
}
