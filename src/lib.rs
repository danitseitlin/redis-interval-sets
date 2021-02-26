#[macro_use]
extern crate redis_module;

use redis_module::native_types::RedisType;
use redis_module::{raw, Context, NextArg, RedisError, RedisResult, REDIS_OK};
use std::os::raw::c_void;

#[derive(Debug, PartialEq)]
struct Set {
    member: String,
    min_score: i64,
    max_score: i64,
}

struct IntervalSet {
    sets: Vec<Set>,
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

fn get_sets<A: NextArg>(mut args: A) -> Result<Vec<Set>, RedisError> {
    let mut sets = vec![];

    while let Ok(member) = args.next_string() {
        let set = Set {
            member,
            // If the user supplied a member, they must provide scores as well:
            min_score: args.next_i64()?,
            max_score: args.next_i64()?,
        };
        sets.push(set);
    }

    Ok(sets)
}

fn is_set(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;

    let sets = get_sets(&mut args)?;
    if sets.is_empty() {
        return Err(RedisError::WrongArity);
    }

    let key = ctx.open_key_writable(&key);

    match key.get_value::<IntervalSet>(&REDIS_INTERVAL_SETS)? {
        Some(value) => {
            println!("Count of items before new item: {}", value.sets.len());
            value.sets.extend(sets);
            println!("Count of items: {}", value.sets.len());
        }
        None => {
            println!("Creating a new key");
            let value = IntervalSet { sets };
            println!("Count of items: {}", value.sets.len());
            key.set_value(&REDIS_INTERVAL_SETS, value)?;
        }
    }

    REDIS_OK
}

fn is_del(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;

    let key = ctx.open_key_writable(&key);

    match key.get_value::<IntervalSet>(&REDIS_INTERVAL_SETS)? {
        Some(_value) => key.delete(),
        None => Ok(().into()),
    };

    REDIS_OK
}

fn is_get(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;

    let key = ctx.open_key(&key);

    println!("is.get on key");

    match key.get_value::<IntervalSet>(&REDIS_INTERVAL_SETS)? {
        Some(value) => {
            if let Ok(member) = args.next_string() {
                let sets: Vec<_> = value
                    .sets
                    .iter()
                    .filter(|set| set.member == member)
                    .map(|set| vec![set.min_score.clone().to_string(), set.max_score.clone().to_string()])
                    .collect();

                return Ok(sets.into());
            }
            else {
                let sets: Vec<_> = value
                    .sets
                    .iter()
                    .filter(|_set| true)
                    .map(|set| vec![set.member.clone(), set.min_score.clone().to_string(), set.max_score.clone().to_string()])
                    .collect();

                return Ok(sets.into());
            }
        }
        None => Ok(().into()),
    }
}

fn is_find(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;
    let score = args.next_i64()?;
    let key = ctx.open_key(&key);

    return match key.get_value::<IntervalSet>(&REDIS_INTERVAL_SETS)? {
        Some(value) => {
            let sets: Vec<_> = value
                .sets
                .iter()
                .filter(|set| set.min_score <= score && set.max_score >= score)
                .map(|set| set.member.clone())
                .collect();
            Ok(sets.into())
        }
        None => Ok(().into()),
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
        ["is.set", is_set, "write", 1, 1, 1],
        ["is.get", is_get, "readonly", 1, 1, 1],
        ["is.find", is_find, "readonly", 1, 1, 1],
        ["is.del", is_del, "write", 1, 1, 1]
    ],
}

//////////////////////////////////////////////////////

#[test]
fn test_get_sets_empty() {
    let args = vec![];
    let sets = get_sets(args.into_iter());
    let sets = sets.expect("no sets");
    assert_eq!(sets, vec![]);
}

#[test]
fn test_get_sets_partial1() {
    let args = vec!["member1".to_string()];
    let sets = get_sets(args.into_iter());
    match sets.expect_err("should fail on partial arguments") {
        RedisError::WrongArity => {}
        _ => panic!("wrong error"),
    }
}

#[test]
fn test_get_sets_partial2() {
    let args = vec!["member1".to_string(), "10".to_string()];
    let sets = get_sets(args.into_iter());
    match sets.expect_err("should fail on partial arguments") {
        RedisError::WrongArity => {}
        _ => panic!("wrong error"),
    }
}

#[test]
fn test_get_sets_single() {
    let args = vec!["member1".to_string(), "10".to_string(), "20".to_string()];
    let sets = get_sets(args.into_iter());
    let sets = sets.expect("one member");
    assert_eq!(
        sets,
        vec![Set {
            member: "member1".to_string(),
            min_score: 10,
            max_score: 20,
        }]
    );
}

#[test]
fn test_get_sets_multi() {
    let args = vec![
        "member1".to_string(),
        "10".to_string(),
        "20".to_string(),
        "member2".to_string(),
        "30".to_string(),
        "40".to_string(),
    ];
    let sets = get_sets(args.into_iter());
    let sets = sets.expect("multiple members");
    assert_eq!(
        sets,
        vec![
            Set {
                member: "member1".to_string(),
                min_score: 10,
                max_score: 20,
            },
            Set {
                member: "member2".to_string(),
                min_score: 30,
                max_score: 40,
            }
        ]
    );
}
