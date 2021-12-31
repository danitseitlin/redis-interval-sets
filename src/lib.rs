#[macro_use]
extern crate redis_module;
pub mod structs;
use redis_module::native_types::RedisType;
use redis_module::{raw, Context, NextArg, RedisError, RedisResult, REDIS_OK};
use std::os::raw::c_void;
use structs::{Set, IntervalSet};

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

fn get_members<A: NextArg>(mut args: A) -> Result<Vec<String>, RedisError> {
    let mut members = vec![];

    while let Ok(member) = args.next_string() {
        members.push(member);
    }

    Ok(members)
}

fn is_in_score_range(set: &&Set, score: i64) -> bool {
    if set.min_score <= score && set.max_score >= score {
        return true;
    }
    return false;
}

fn is_add(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key_name_arg = args.next_string()?;

    let sets = get_sets(&mut args)?;
    if sets.is_empty() {
        return Err(RedisError::WrongArity);
    }

    let key = ctx.open_key_writable(&key_name_arg);
    match key.get_value::<IntervalSet>(&REDIS_INTERVAL_SETS)? {
        Some(value) => {
            println!("[iset.add] Updating key '{}'", key_name_arg);
            value.sets.extend(sets);
        }
        None => {
            println!("[iset.add] Adding a new key '{}'", key_name_arg);
            let value = IntervalSet { sets };
            key.set_value(&REDIS_INTERVAL_SETS, value)?;
        }
    }
    

    REDIS_OK
}

fn is_del(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key_name_arg = args.next_string()?;
    let members = get_members(&mut args)?;
    let key = ctx.open_key_writable(&key_name_arg);
    return match key.get_value::<IntervalSet>(&REDIS_INTERVAL_SETS)? {
        Some(value) => {
            println!("[iset.del] Deleting key '{}'", key_name_arg);
            if members.is_empty() {
                key.delete()?;
                return REDIS_OK;
            }
            for member in members {
                value.sets.retain(|set| set.member != member)
            }
            return REDIS_OK;
        }
        None => Ok(().into()),
    };
}

fn is_get(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key_name_arg = args.next_string()?;
    let key = ctx.open_key(&key_name_arg);
    match key.get_value::<IntervalSet>(&REDIS_INTERVAL_SETS)? {
        Some(value) => {
            println!("[is.get] Retrieving key '{}'", key_name_arg);
            if let Ok(member) = args.next_string() {
                println!("[is.get] Retrieving key '{}' members '{}'", key_name_arg, member);
                let sets: Vec<_> = value
                    .sets
                    .iter()
                    .filter(|set| set.member == member)
                    .map(|set| {
                        vec![
                            set.min_score.clone().to_string(),
                            set.max_score.clone().to_string(),
                        ]
                    })
                    .collect();

                return Ok(sets.into());
            } else {
                let sets: Vec<_> = value
                    .sets
                    .iter()
                    .filter(|_set| true)
                    .map(|set| {
                        vec![
                            set.member.clone(),
                            set.min_score.clone().to_string(),
                            set.max_score.clone().to_string(),
                        ]
                    })
                    .collect();

                return Ok(sets.into());
            }
        }
        None => Ok(().into()),
    }
}

fn is_score(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key_name_arg = args.next_string()?;
    let score = args.next_i64()?;
    let key = ctx.open_key(&key_name_arg);
    println!("[iset.score] Retrieving key '{}' sets in score '{}'", key_name_arg, score);
    return match key.get_value::<IntervalSet>(&REDIS_INTERVAL_SETS)? {
        Some(value) => {
            let sets: Vec<_> = value
                .sets
                .iter()
                .filter(|set| is_in_score_range(set, score) == true)
                .map(|set| set.member.clone())
                .collect();
            Ok(sets.into())
        }
        None => Ok(().into()),
    };
}

fn is_not_score(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key_name_arg = args.next_string()?;
    let score = args.next_i64()?;
    let key = ctx.open_key(&key_name_arg);

    println!("[iset.not_score] Retrieving key '{}' sets not in score '{}'", key_name_arg, score);
    return match key.get_value::<IntervalSet>(&REDIS_INTERVAL_SETS)? {
        Some(value) => {
            let sets: Vec<_> = value
                .sets
                .iter()
                .filter(|set| is_in_score_range(set, score) == false)
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
        ["iset.add", is_add, "write", 1, 1, 1],
        ["iset.del", is_del, "write", 1, 1, 1],
        ["iset.get", is_get, "readonly", 1, 1, 1],
        ["iset.score", is_score, "readonly", 1, 1, 1],
        ["iset.not_score", is_not_score, "readonly", 1, 1, 1],
    ],
}

#[test]
fn get_sets_empty() {
    let args = vec![];
    let sets = get_sets(args.into_iter());
    let sets = sets.expect("no sets");
    assert_eq!(sets, vec![]);
}

#[test]
fn get_sets_partial1() {
    let args = vec!["member1".to_string()];
    let sets = get_sets(args.into_iter());
    match sets.expect_err("should fail on partial arguments") {
        RedisError::WrongArity => {}
        _ => panic!("wrong error"),
    }
}

#[test]
fn get_sets_partial2() {
    let args = vec!["member1".to_string(), "10".to_string()];
    let sets = get_sets(args.into_iter());
    match sets.expect_err("should fail on partial arguments") {
        RedisError::WrongArity => {}
        _ => panic!("wrong error"),
    }
}

#[test]
fn get_sets_single() {
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
fn get_sets_multi() {
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

#[test]
fn get_members_empty() {
    let args = vec![];
    let members = get_members(args.into_iter());
    let members = members.expect("no members");
    let empty_list: Vec<String> = vec![];
    assert_eq!(members, empty_list);
}

#[test]
fn get_members_single() {
    let args = vec!["member1".to_string()];
    let members = get_members(args.into_iter());
    let members = members.expect("one member");
    assert_eq!(
        members,
        vec!["member1"]
    );
}

#[test]
fn get_members_multi() {
    let args = vec![
        "member1".to_string(),
        "member2".to_string(),
    ];
    let members = get_members(args.into_iter());
    let members = members.expect("multiple members");
    assert_eq!(
        members,
        vec!["member1".to_string(), "member2".to_string()]
    );
}