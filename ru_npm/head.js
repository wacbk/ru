#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
var LIB, arch, platform, require, suffix;

import NodeCls from '@w5/node_cls';

import {
  createRequire
} from 'module';

require = createRequire(import.meta.url);

({platform, arch} = process);

if (platform === 'linux') {
  if (process.report.getReport().header.glibcVersionRuntime) {
    suffix = 'glibc';
  } else {
    suffix = 'musl';
  }
  platform += '-' + suffix;
}

LIB = require(`@w5/ru-${arch}-${platform}`);
