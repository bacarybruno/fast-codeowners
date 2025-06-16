import { Bench } from 'tinybench'
import JSCodeowners from 'codeowners'

import { Codeowners as NativeCodeowners } from '../index.js'

const b = new Bench({
  name: 'Check codeowners',
})

b.add('fast-codeowners', () => {
  const codeowners = new NativeCodeowners()
  codeowners.getOwners('index.ts')
})

b.add('codeowners (npm', () => {
  const codeowners = new JSCodeowners()
  codeowners.getOwner('index.ts')
})

await b.run()

console.table(b.table())
