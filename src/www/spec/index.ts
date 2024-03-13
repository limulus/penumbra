import './mocha.js'

mocha.setup('bdd')
await import('./tests.js')
mocha.run()
