#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
import NodeCls from '@w5/node_cls';

import lib from './lib.node';

export default NodeCls(lib);
