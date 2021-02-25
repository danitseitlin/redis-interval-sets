#[macro_use]
extern crate redis_module;

use redis_module::native_types::RedisType;
use redis_module::{raw, Context, NextArg, RedisResult, RedisValue};
use std::os::raw::c_void;

#[derive(Debug)]
struct Set {
    member: String,
    min_score: i64,
    max_score: i64
}

struct IntervalSet {
    sets: Vec<Set>
}

static REDIS_INTERVAL_SETS: RedisType = RedisType::new(
    "IntervlSt",
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
    Box::from_raw(value as *mut IntervalSet);
}

fn is_add(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;
    let member = args.next_string()?;
    let min_score = args.next_i64()?;
    let max_score = args.next_i64()?;

    let key = ctx.open_key_writable(&key);
    
    match key.get_value::<IntervalSet>(&REDIS_INTERVAL_SETS)? {
        Some(value) => {
            let set = Set {
                member: member,
                min_score: min_score,
                max_score: max_score
            };
            println!("Count of items before new item: {}", value.sets.len());
            value.sets.push(set);
            println!("Count of items: {}", value.sets.len());
        }
        None => {
            println!("Creating a new key");
            let mut value = IntervalSet {
                sets: vec![]
            };
            value.sets.push(Set {
                member: member,
                min_score: min_score,
                max_score: max_score
            });
            
            println!("Count of items: {}", value.sets.len());
            key.set_value(&REDIS_INTERVAL_SETS, value)?;
        }
    }

    Ok(RedisValue::SimpleStringStatic("OK"))
}

fn is_get(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;

    let key = ctx.open_key(&key);
    println!("is.get on key");
    return match key.get_value::<IntervalSet>(&REDIS_INTERVAL_SETS)? {
        Some(value) => {
            println!("Fetching all sets for key");
            let mut sets: Vec<RedisValue> = vec![];
            println!("Sets: {}", value.sets.len());
            for set in value.sets.iter() {
                println!("Found member {}", set.member);
                sets.push(RedisValue::SimpleString(set.member.clone()))
            }
            return Ok(RedisValue::Array(sets))
        },
        None => Ok(RedisValue::Null)
    };
}

fn is_filter(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;
    let score = args.next_i64()?;
    let key = ctx.open_key(&key);

    return match key.get_value::<IntervalSet>(&REDIS_INTERVAL_SETS)? {
        Some(value) => {
            let mut list: Vec<RedisValue> = vec![];
            for set in value.sets.iter() {
                if set.min_score <= score && set.max_score >= score {
                    list.push(RedisValue::SimpleString(set.member.clone()))
                }
            }
            Ok(RedisValue::Array(list))
        },
        None => Ok(RedisValue::Null)
    };
}

//////////////////////////////////////////////////////

redis_module! {
    name: "interval_set",
    version: 1,
    data_types: [
        REDIS_INTERVAL_SETS
    ],
    commands: [
        ["is.add", is_add, "write", 1, 1, 1],
        ["is.get", is_get, "readonly", 1, 1, 1],
        ["is.filter", is_filter, "readonly", 1, 1, 1]
    ],
}