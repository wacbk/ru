#!/usr/bin/env coffee

> @w5/node_cls
  module > createRequire

require = createRequire(import.meta.url)

{platform, arch } = process

if platform == 'linux'
  if process.report.getReport().header.glibcVersionRuntime
    suffix = 'glibc'
  else
    suffix = 'musl'
  platform += '-'+suffix

LIB = require("@w5/ru-#{arch}-#{platform}")

