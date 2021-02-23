#[macro_use]
extern crate redis_module;

use redis_module::native_types::RedisType;
use redis_module::{raw, Context, NextArg, RedisResult};
use std::os::raw::c_void;

#[derive(Debug)]
struct IntervalSetType {
    sets: SetType[]
}

struct SetType {
    member: String,
    min_score: Number,
    max_score: Number
}

static REDIS_INTERVAL_SETS: RedisType = RedisType::new(
    "IntervalSetType",
    0,
    raw::RedisModuleTypeMethods {
        version: raw::REDISMODULE_TYPE_METHOD_VERSION as u64,
        rdb_load: None,
        rdb_save: None,
        aof_rewrite: None,
        free: Some(free),

        // Currently unused by Redis
        mem_usage: None,
        digest: None,

        // Aux data
        aux_load: None,
        aux_save: None,
        aux_save_triggers: 0,
    },
);

unsafe extern "C" fn free(value: *mut c_void) {
    Box::from_raw(value as *mut IntervalSetType);
}


fn is_add(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;
    let member = args.next_string()?;
    let min_score = args.next_i64()?;
    let max_score = args.next_i64()?;

    let key = ctx.open_key_writable(&key);
    
    match key.get_value::<IntervalSetType>(&REDIS_INTERVAL_SETS)? {
        Some(value) => {
            value.sets.push({
                member: member,
                min_score: min_score,
                max_score: max_score
            })
            key.set_value(&REDIS_INTERVAL_SETS, value)?;
        }
        None => {
            let value = IntervalSetType {
                sets: {
                    member: member,
                    min_score: min_score,
                    max_score: max_score
                }
            };

            key.set_value(&REDIS_INTERVAL_SETS, value)?;
        }
    }

    Ok(size.into())
}

fn is_get(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;

    let key = ctx.open_key(&key);

    let value = match key.get_value::<IntervalSetType>(&REDIS_INTERVAL_SETS)? {
        Some(value) => value.sets.as_str().into(),
        None => ().into(),
    };

    Ok(value)
}

/*
fn alloc_set(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;
    let size = args.next_i64()?;

    ctx.log_debug(format!("key: {}, size: {}", key, size).as_str());

    let key = ctx.open_key_writable(&key);

    match key.get_value::<MY_REDIS_TYPE>(&MY_REDIS_TYPE)? {
        Some(value) => {
            value.data = "B".repeat(size as usize);
        }
        None => {
            let value = IntervalSetType {
                data: "A".repeat(size as usize),
            };

            key.set_value(&MY_REDIS_TYPE, value)?;
        }
    }

    Ok(size.into())
}

fn alloc_get(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;

    let key = ctx.open_key(&key);

    let value = match key.get_value::<IntervalSetType>(&MY_REDIS_TYPE)? {
        Some(value) => value.data.as_str().into(),
        None => ().into(),
    };

    Ok(value)
}*/

//////////////////////////////////////////////////////

redis_module! {
    name: "interval_set",
    version: 1,
    data_types: [
        REDIS_INTERVAL_SETS
    ],
    commands: [
        ["is.add", is_add, "write", 1, 1, 1],
        ["is.get", is_get, "readonly", 1, 1, 1]
        //["alloc.set", alloc_set, "write", 1, 1, 1],
        //["alloc.get", alloc_get, "readonly", 1, 1, 1],
    ],
}