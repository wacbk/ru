#!/usr/bin/env coffee

> ./lib.js


{
  REDIS_HOST_PORT
  REDIS_USER
  REDIS_PORT
  REDIS_PASSWORD
} = process.env

[REDIS_HOST, REDIS_PORT] = REDIS_HOST_PORT.split(':')

{
  serverHostPort
  Redis
} = lib
server = serverHostPort REDIS_HOST, parseInt(REDIS_PORT)

redis = await Redis(
  server
  0 # parseInt REDIS_DB
  'default' # REDIS_USER
  REDIS_PASSWORD
)
#
# key = "testz"
#
# await redis.hset key,"a","b"
# await redis.hset key,"c","d"
# console.log await redis.hmget key,"a","b","c"
# await redis.del key




#console.log await redis.testz key
#console.log await redis.zrevrangebyscore key
#console.log await redis.zrevrangebyscoreWithscores key
#console.log 'done'

# key = 'test'
#key = Buffer.from [1,2,3]
#console.log await redis.get(key)
#console.log await redis.set(key,'123')
#console.log await redis.get(key)
#console.log await redis.set(key, 332)
#console.log await redis.get(key)
#console.log await redis.del(key)
#console.log await redis.get(key)
#console.log await redis.setex(key,'123',3)
####
####
#{
#  u64Bin
#  binU64
#  passwordHash
#} = lib
#
#sleep = =>
#  new Promise((resolve) => setTimeout(resolve, 10))
#
#minute = =>
#  parseInt new Date()/6e4
#
#main = =>
#  Promise.all [
#    passwordHash u64Bin(1)
#    passwordHash Buffer.from([0]),Buffer.from([2])
#    passwordHash 'a'
#    passwordHash Buffer.from([97])
#    passwordHash new Uint8Array([97])
#  ]
#
#cost = (p)=>
#  begin = new Date
#  r = await p
#  console.log r, 'cost',Math.round(new Date()-begin)/1000
#
#  r
#
#do =>
#  await cost main()
#  await cost passwordHash new Uint8Array([97])
#  return
#  begin = minute()
#  {rss} = process.memoryUsage()
#  n = 0
#  pre = 0
#  loop
#    await main()
#    if ++n%100 == 1
#      gc()
#      await sleep()
#
#      leak = parseInt((process.memoryUsage().rss-rss)/1024/1024)
#      if leak != pre
#        pre = leak
#        console.log(
#          minute()-begin,'minute'
#          n,'loop'
#          'leak', leak,'MB'
#        )
#  return
###
