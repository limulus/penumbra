const { EleventyHtmlBasePlugin } = require('@11ty/eleventy')
const pluginRss = require('@11ty/eleventy-plugin-rss')
const pluginWebc = require('@11ty/eleventy-plugin-webc')
const hljs = require('highlight.js')
const markdownIt = require('markdown-it')
const markdownItFootnote = require('markdown-it-footnote')
const markdownItTaskLists = require('markdown-it-task-lists')

/** @param {import("@11ty/eleventy").UserConfig} eleventyConfig */
module.exports = function (eleventyConfig) {
  const input = 'www'
  const output = 'dist/www'
  const layouts = '_includes/layouts'
  const components = `${input}/_includes/components/**/*.webc`

  eleventyConfig.addGlobalData('site', { url: 'https://limulus.net/penumbra' })

  eleventyConfig.addPassthroughCopy(`${input}/assets`, { expand: true })
  eleventyConfig.addPassthroughCopy(`${input}/**/*.{png,svg,jpg,jpeg}`)

  eleventyConfig.addPlugin(EleventyHtmlBasePlugin)
  eleventyConfig.addPlugin(pluginRss)
  eleventyConfig.addPlugin(pluginWebc, { components })

  const md = markdownIt({
    html: true,
    linkify: true,
    highlight: (str, language) => {
      if (language && hljs.getLanguage(language)) {
        return (
          '<pre><code class="hljs">' +
          hljs.highlight(str, { language, ignoreIllegals: true }).value +
          '</code></pre>'
        )
      }
      return '' // use external default escaping
    },
  })
  md.use(markdownItFootnote)
  md.use(markdownItTaskLists)
  eleventyConfig.setLibrary('md', md)

  eleventyConfig.setServerOptions({
    domDiff: false,
    middleware: [
      (_req, res, next) => {
        res.setHeader('Cross-Origin-Opener-Policy', 'same-origin')
        res.setHeader('Cross-Origin-Embedder-Policy', 'require-corp')
        next()
      },
    ],
    watch: ['dist/www/assets/**/*.{css,js}'],
  })

  return { dir: { input, output, layouts }, pathPrefix: '/penumbra/' }
}
