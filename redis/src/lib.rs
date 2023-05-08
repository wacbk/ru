mod init;
use fred::{
  interfaces::{
    ClientLike, FunctionInterface, HashesInterface, KeysInterface, SetsInterface,
    SortedSetsInterface,
  },
  pool::RedisPool,
  prelude::{ReconnectPolicy, RedisClient, RedisConfig, ServerConfig as Config},
  types::{Expiration, RedisMap, Server, SetOptions, ZRange, ZRangeBound, ZRangeKind},
};
pub use init::init;
use nlib::*;

alias!(ServerConfig, Config);
alias!(Redis, RedisClient);
as_value_cls!(ServerConfig, Redis);

fn min_max_score(cx: &'_ mut Cx) -> Result<(ZRange, ZRange), Throw> {
  let len = cx.len();
  let min = if len > 2 {
    to_zrange(cx, 2)?
  } else {
    ZRange {
      kind: ZRangeKind::Inclusive,
      range: ZRangeBound::NegInfiniteScore,
    }
  };
  let max = if len > 3 {
    to_zrange(cx, 3)?
  } else {
    ZRange {
      kind: ZRangeKind::Inclusive,
      range: ZRangeBound::InfiniteScore,
    }
  };
  Ok((min, max))
}

fn max_min_score(cx: &'_ mut Cx) -> Result<(ZRange, ZRange), Throw> {
  let len = cx.len();

  let max = if len > 2 {
    to_zrange(cx, 2)?
  } else {
    ZRange {
      kind: ZRangeKind::Inclusive,
      range: ZRangeBound::InfiniteScore,
    }
  };
  let min = if len > 3 {
    to_zrange(cx, 3)?
  } else {
    ZRange {
      kind: ZRangeKind::Inclusive,
      range: ZRangeBound::NegInfiniteScore,
    }
  };
  Ok((max, min))
}

pub fn to_zrange(cx: &'_ mut Cx, n: usize) -> Result<ZRange, Throw> {
  let val = cx.argument::<JsValue>(n)?;
  Ok(if val.is_a::<JsString, _>(cx) {
    val.downcast_or_throw::<JsString, _>(cx)?.value(cx).into()
  } else {
    val
      .downcast_or_throw::<JsNumber, _>(cx)?
      .value(cx)
      .try_into()
      .unwrap()
  })
}

fn limit_offset(cx: &mut FunctionContext, n: usize) -> Result<Option<(i64, i64)>, Throw> {
  let len = cx.len();
  Ok(if len > n {
    let limit = as_f64(cx, n)? as i64;
    let n = n + 1;
    let offset = if len > n { as_f64(cx, n)? as i64 } else { 0 };
    Some((offset, limit))
  } else {
    None
  })
}

js_fn! {

  server_host_port |cx| {
      let host = to_str(cx, 0)?;
      let port = as_f64(cx, 1)? as u16;
      ServerConfig(Config::Centralized {
          server:Server {
              host:host.into(), port, tls_server_name:None
      }
      })
  }

  server_cluster |cx| {
      ServerConfig(Config::Clustered {
          hosts:to_kvli(
                    cx,
                    0,
                    jsval2num::<u16>
                )?.iter().map(
                |(host,port)| Server {host:host.into(), port:*port, tls_server_name:None}
                ).collect()
      })
  }

  redis_new |cx| {
      let mut conf = RedisConfig { version: fred::types::RespVersion::RESP3, ..Default::default() };
      let server = (*cx.argument::<JsBox<ServerConfig>>(0)?).clone();
      conf.server = server;
      let database = as_f64(cx, 1)? as u8;
      if database != 0 {
          conf.database = Some(database);
      }
      conf.username = Some(to_str(cx, 2)?);
      conf.password = Some(to_str(cx, 3)?);
      let policy = ReconnectPolicy::new_exponential(0, 100, 30_000, 2);

      r#await(
          cx,
          async move {
              let client = RedisClient::new(
                  conf,
                  None,
                  Some(policy)
              );
              //let client = RedisPool::new(conf, None, Some(policy), 3)?;
              client.connect();
              client.wait_for_connect().await?;
              Ok(client)
          },
          |mut cx, client| Ok(Redis(client).as_value(&mut cx)),
      )?
  }
}

macro_rules! fcall_ro {
    ($cx:ident, $ty:ty)=>{{
        let name = to_str($cx, 1)?;
        let keys = to_bin_li($cx, 2)?;
        let vals = to_bin_li($cx, 3)?;
        this!($cx this {
            this.fcall_ro::<$ty,_,_,_>(
                name,
                keys,
                vals,
            )
        })
    }}
}

macro_rules! fcall{
    ($cx:ident, $ty:ty)=>{{
        let name = to_str($cx, 1)?;
        let keys = to_bin_li($cx, 2)?;
        let vals = to_bin_li($cx, 3)?;
        if keys.len() > 0{
            this!($cx this {
                this.fcall::<$ty,_,_,_>(
                    name,
                    keys,
                    vals,
                )
            })
        } else {
            this!($cx this {
                this.fcall_ro::<$ty,_,_,_>(
                    name,
                    keys,
                    vals,
                )
            })
        }
    }}
}

#[macro_export]
macro_rules! def_fn {
  ($fn:ident |$cx:ident| $body:tt) => {
    nlib::paste! {
      pub fn $fn(mut $cx: Cx) -> JsResult<JsValue> {
        let $cx = &mut $cx;
        $body
      }
    }
  };

  ($($fn:ident |$cx:ident| $body:block)+) => {
    $(
      def_fn!($fn |$cx| $body);
    )+
  }
}

macro_rules! this {
  ($cx:ident $this:ident $body:block) => {{
    let $this = $cx.argument::<JsBox<Redis>>(0)?.0.clone();
    let r = jswait!($cx, $body);
    r
  }};
}

def_fn! {

  redis_quit |cx| {
      this!(cx this {
          async move {
              this.quit().await?;
              //this.quit_pool().await;
              Ok::<_,anyhow::Error>(())
          }
      })
  }

  redis_get_b |cx| {
      let a1=to_bin(cx, 1)?;
      this!(cx this {
          this.get::<Option<Vec<u8>>, _>(a1)
      })
  }

  redis_set |cx| {
      let a1 = to_bin(cx, 1)?;
      let a2 = to_bin(cx, 2)?;
      this!(cx this {
          this.set::<(),_,_>(
              a1,
              a2,
              None,
              None,
              false
          )
      })
  }

  redis_setex |cx| {
      let a1 = to_bin(cx, 1)?;
      let a2 = to_bin(cx, 2)?;
      let a3 = as_f64(cx, 3)?;
      this!(cx this  {
          this.set::<(),_,_>(
              a1,a2,
              Some(Expiration::EX(a3 as _)),
              None,
              false
          )
      })
  }

  redis_expire |cx| {
      let a1 = to_bin(cx, 1)?;
      let a2=as_f64(cx, 2)?;
      this!(cx this {
          this.expire::<bool,_>(
              a1,
              a2 as _
          )
      })
  }

  redis_del |cx| {
      let a1 = args_bin_li(cx,1)?;
      this!(cx this {
          this.del::<u32,_>(a1)
      })
  }

  redis_exist |cx| {
      let a1 = args_bin_li(cx,1)?;
      this!(cx this {
          this.exists::<u32,_>(a1)
      })
  }

  redis_hmget_s |cx| {
      let a1=to_bin(cx, 1)?;
      let a2=args_bin_li(cx,2)?;
      this!(cx this {
          this.hmget::<Vec<Option<String>>,_,_>(a1,a2)
      })
  }

  redis_hmget |cx| {
      let a1=to_bin(cx, 1)?;
      let a2=args_bin_li(cx,2)?;
      this!(cx this {
          this.hmget::<Vec<Option<Vec<u8>>>,_,_>(a1,a2)
      })
  }

  redis_hmget_n |cx| {
      let a1=to_bin(cx, 1)?;
      let a2=args_bin_li(cx,2)?;
      this!(cx this {
          this.hmget::<Vec<Option<f64>>,_,_>(a1,a2)
      })
  }

  redis_hget |cx| {
      let a1=to_bin(cx, 1)?;
      let a2=to_bin(cx, 2)?;
      this!(cx this {
          this.hget::<Option<String>,_,_>(a1,a2)
      })
  }

  redis_hget_b |cx| {
      let a1=to_bin(cx, 1)?;
      let a2=to_bin(cx, 2)?;
      this!(cx this {
          this.hget::<Option<Vec<u8>>,_,_>(
              a1,
              a2,
          )
      })
  }

  redis_hget_n |cx| {
      let a1=to_bin(cx, 1)?;
      let a2=to_bin(cx, 2)?;
      this!(cx this {
          this.hget::<Option<f64>,_,_>(
              a1,
              a2,
          )
      })
  }

  redis_hset |cx| {
      let a1=to_bin(cx, 1)?;
      let val: RedisMap = if cx.len() == 3 {
        ok!(cx,to_kvli(cx, 2, jsval2bin)?.try_into())
      } else {
        ok!(cx,(to_bin(cx, 2)?, to_bin(cx, 3)?).try_into())
      };
      this!(cx this {
          this.hset::<(),_,_>(a1, val)
      })
  }

  redis_hincrby |cx| {
      let a1=to_bin(cx, 1)?;
      let a2=to_bin(cx, 2)?;
      let a3=as_f64(cx, 3)?;
      this!(cx this {
          this.hincrby::<f64,_,_>(
              a1,
              a2,
              a3 as _,
          )
      })
  }

  redis_hincr |cx| {
      let a1=to_bin(cx, 1)?;
      let a2=to_bin(cx, 2)?;
      this!(cx this {
          this.hincrby::<f64,_,_>(
              a1,
              a2,
              1
          )
      })
  }

  redis_hexist |cx| {
      let a1=to_bin(cx, 1)?;
      let a2=to_bin(cx, 2)?;
      this!(cx this {
          this.hexists::<bool,_,_>(
              a1,
              a2,
          )
      })
  }

  redis_smembers |cx| {
      let a1 = to_bin(cx, 1)?;
      this!(cx this {
          this.smembers::<Vec<Vec<u8>>,_>(a1)
      })
  }

  redis_sadd |cx| {
      let a1=to_bin(cx, 1)?;
      let a2=args_bin_li(cx, 2)?;
      this!(cx this {
          this.sadd::<f64,_,_>(a1,a2)
      })
  }

  redis_zscore |cx| {
      let a1 = to_bin(cx, 1)?;
      let a2 = to_bin(cx, 2)?;

      this!(cx this {
          this.zscore::<Option<f64>,_,_>(a1,a2)
      })
  }

  redis_zincrby |cx| {
      let a1=to_bin(cx, 1)?;
      let a3=as_f64(cx, 3)?;
      let a2=to_bin(cx, 2)?;
      this!(cx this {
          this.zincrby::<f64,_,_>(
              a1,a3,a2
          )
      })
  }

  redis_zincr |cx| {
      let a1 = to_bin(cx, 1)?;
      let a2 = to_bin(cx, 2)?;
      this!(cx this {
          this.zincrby::<f64,_,_>(
              a1,
              1.0,
              a2,
          )
      })
  }

  // args : key,min,max,[limit],[offset]
  redis_zrangebyscore |cx| {
      let a1 = to_bin(cx, 1)?;
      let a2 = limit_offset(cx,4)?;
      let (min,max) = min_max_score(cx)?;
      this!(cx this {
          this.zrangebyscore::<Vec<Vec<u8>>,_,_,_>(
              a1,
              min,
              max,
              false,
              a2
          )
      })
  }

  redis_zrangebyscore_withscores |cx| {
      let a1 = to_bin(cx, 1)?;
      let a2 = limit_offset(cx,4)?;
      let (min,max) = min_max_score(cx)?;
      this!(cx this {
          this.zrangebyscore::<Vec<(Vec<u8>,f64)>,_,_,_>(
              a1,
              min,
              max,
              true,
              a2
          )
      })
  }

  redis_zrevrangebyscore |cx| {
      let a1 = to_bin(cx, 1)?;
      let a2 = limit_offset(cx,4)?;
      let (max,min) = max_min_score(cx)?;
      this!(cx this {
          this.zrevrangebyscore::<Vec<Vec<u8>>,_,_,_>(
              a1,
              max,
              min,
              false,
              a2
          )
      })
  }

  redis_zrevrangebyscore_withscores |cx| {
      let a1 = to_bin(cx, 1)?;
      let a2 = limit_offset(cx,4)?;
      let (max,min) = max_min_score(cx)?;
      this!(cx this {
          this.zrevrangebyscore::<Vec<(Vec<u8>,f64)>,_,_,_>(
              a1,
              max,
              min,
              true,
              a2
          )
      })
  }

  redis_zrem |cx| {
      let a1 = to_bin(cx, 1)?;
      let a2 = args_bin_li(cx, 2)?;

      this!(cx this {
          this.zrem::<f64,_,_>(
              a1,
              a2
          )
      })
  }

  redis_zadd |cx| {
      let a1 = to_bin(cx, 1)?;
      let a2 = to_bin(cx, 2)?;
      let a3 = as_f64(cx, 3)?;
      this!(cx this {
          this.zadd::<f64,_,_>(
              a1,
              None,
              None,
              false,
              false,
              (
                  a3,
                  a2,
              )
          )
      })
  }

  redis_zadd_xx |cx| {
      let a1 = to_bin(cx, 1)?;
      let a2 = to_bin(cx, 2)?;
      let a3 = as_f64(cx, 3)?;

      this!(cx this {
          this.zadd::<f64,_,_>(
              a1,
              Some(SetOptions::XX),
              None,
              false,
              false,
              (
                  a3,
                  a2,
              )
          )
      })
  }


  redis_fcall |cx| { fcall!(cx,()) }
  redis_fcall_r |cx| { fcall_ro!(cx,()) }
  redis_fbool |cx| { fcall!(cx,Option<bool>) }
  redis_fbool_r |cx| { fcall_ro!(cx,Option<bool>) }
  redis_fbin |cx| { fcall!(cx,Option<Vec<u8>>) }
  redis_fbin_r |cx| { fcall_ro!(cx,Option<Vec<u8>>) }
  redis_fnum |cx| { fcall!(cx,Option<f64>) }
  redis_fnum_r |cx| { fcall_ro!(cx,Option<f64>) }
  redis_fstr |cx| { fcall!(cx,Option<String>) }
  redis_fstr_r |cx| { fcall_ro!(cx,Option<String>) }

  redis_get |cx| {
    let a1 = to_bin(cx, 1)?;
    this!(cx this { this.get::<Option<String>, _>(a1) })
  }

  redis_fnload |cx| {
    let a1 = to_str(cx, 1)?;
    this!(cx this { this.function_load::<String, _>(true, a1) })
  }

}
