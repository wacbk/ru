#!/usr/bin/env coffee

> zx/globals:
  @w5/uridir
  @w5/read
  @w5/write
  @w5/extract > extract
  @w5/verincr
  @w5/coffee_plus
  coffeescript
  path > join

CoffeePlus(coffeescript)

ROOT = uridir(import.meta)

RU = join ROOT, 'ru'

cd RU
await $'npx cep -c src -o lib'
await $'pnpm run build'
cd ROOT
await $'./sh/gen.init.coffee ru'

save = =>
  pkg.version = version
  write(
    pkg_fp
    JSON.stringify pkg
  )
  return

pkg_fp = join(ROOT,'ru/package.json')

{dependencies} = pkg = JSON.parse read pkg_fp

version = verincr pkg.version

save()

pkg_fp = join(ROOT,'ru_npm/package.json')

{
  optionalDependencies
} = pkg = JSON.parse read pkg_fp

for i of optionalDependencies
  optionalDependencies[i] = version
pkg.dependencies = dependencies

save()

RU_NPM = RU+'_npm'

write(
  join RU_NPM, 'index.js'
  coffeescript.compile(
    read join RU_NPM,'head.coffee'
    bare:true
  ).split('\n')[1..].join('\n')+'export const {'+extract(
    read join RU, 'lib/index.js'
    'export const {'
    '}'
  ).replace(/\s+/g,'')+'} = NodeCls(LIB);'

)

await $'gitsync'

