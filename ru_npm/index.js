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
export const {zipU64,unzipU64,unb64,b64,b64U64,u64B64,u64Bin,binU64,passwordHash,z85Load,z85Dump,randomBytes,cookieEncode,cookieDecode,xxh64,xxh32,xxh3B36,ipBin,tld,serverHostPort,serverCluster,svgWebp,Redis} = NodeCls(LIB);