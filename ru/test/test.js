#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
var cost, main, minute, sleep;

import {
  u64Bin,
  binU64,
  passwordHash
} from '..';

sleep = () => {
  return new Promise((resolve) => {
    return setTimeout(resolve, 10);
  });
};

minute = () => {
  return parseInt(new Date() / 6e4);
};

main = () => {
  return Promise.all([passwordHash(u64Bin(1)), passwordHash(Buffer.from([0]), Buffer.from([2])), passwordHash('a'), passwordHash(Buffer.from([97])), passwordHash(new Uint8Array([97]))]);
};

cost = async(p) => {
  var begin, r;
  begin = new Date();
  r = (await p);
  console.log(r, 'cost', Math.round(new Date() - begin) / 1000);
  return r;
};

(async() => {
  var begin, leak, n, pre, rss;
  console.log(u64Bin);
  return;
  await cost(main());
  await cost(passwordHash(new Uint8Array([97])));
  begin = minute();
  ({rss} = process.memoryUsage());
  n = 0;
  pre = 0;
  while (true) {
    await main();
    if (++n % 100 === 1) {
      gc();
      await sleep();
      leak = parseInt((process.memoryUsage().rss - rss) / 1024 / 1024);
      if (leak !== pre) {
        pre = leak;
        console.log(minute() - begin, 'minute', n, 'loop', 'leak', leak, 'MB');
      }
    }
  }
})();
