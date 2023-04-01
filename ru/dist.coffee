#!/usr/bin/env coffee

> zx/globals:
  @w5/uridir
  @w5/read
  @w5/write
  os
  path > join

PACKAGE_JSON = 'package.json'
ROOT = uridir(import.meta)
LIB = join ROOT,'lib'

add = (li, s)=>
  if not li
    return [s]
  if not li.includes s
    li.push s
  li


meta = process.argv[2] or ''

libc = ''

if meta.endsWith '-darwin'
  platform = 'darwin'
else if meta.includes '-linux'
  platform = 'linux'
  suffix = 'musl'
  if meta.endsWith '-'+suffix
    libc = suffix
  else
    libc = 'glibc'
else
  platform = os.platform()

if meta.startsWith 'aarch64-'
  arch = 'arm64'
else if meta.startsWith 'x86_64-'
  arch = 'x64'
else
  arch = os.arch()

do =>
  package_json_fp = join ROOT,PACKAGE_JSON
  package_json = JSON.parse read package_json_fp

  o = {
    ...package_json
  }

  delete o.scripts
  delete o.dependencies
  delete o.devDependencies
  delete o.exports
  delete o.type

  o.name += '-'+arch+'-'+platform

  if libc
    o.name += '-'+libc
    o.libc = [libc]

  o.os = [platform]
  o.cpu = [arch]
  node = 'I.node'
  o.main = './'+node
  o.files = [node]
  cd LIB
  write(
    PACKAGE_JSON
    JSON.stringify(o)
  )
  await $'npm publish --access=public'
  return
