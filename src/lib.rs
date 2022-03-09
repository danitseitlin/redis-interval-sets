#[macro_use]
extern crate redis_module;
pub mod structs;
use redis_module::native_types::RedisType;
use redis_module::{raw, Context, NextArg, RedisError, RedisResult, REDIS_OK, RedisString};
use structs::{Set, Sets, IntervalSet};
use std::os::raw::{c_int, c_void};
use std::ptr::null;
use std::str::FromStr;

static REDIS_INTERVAL_SETS: RedisType = RedisType::new(
    "IntervlSt",
    0,
    raw::RedisModuleTypeMethods {
        version: raw::REDISMODULE_TYPE_METHOD_VERSION as u64,
        rdb_load: Some(rdb_load),
        rdb_save: Some(rdb_save),
        aof_rewrite: None,
        free: Some(free),

        // Currently unused by Redis
        mem_usage: None,
        digest: None,

        // Aux data
        aux_load: None,
        aux_save: None,
        aux_save_triggers: 0,

        free_effort: None,
        unlink: None,
        copy: None,
        defrag: None,
    },
);

unsafe extern "C" fn free(value: *mut c_void) {
    Box::from_raw(value as *mut IntervalSet);
}

unsafe extern "C" fn rdb_save(rdb: *mut raw::RedisModuleIO, value: *mut c_void) {
    let i_sets =  {&*(value as *mut IntervalSet) };
    println!("Saving: {}", &i_sets.to_string());
    raw::save_string(rdb, &i_sets.to_string());
}

pub extern "C" fn rdb_load(rdb: *mut raw::RedisModuleIO, _encver: c_int) -> *mut c_void {
    let value = get_load_data_as_str(rdb);
    Box::into_raw(Box::new(value)).cast::<libc::c_void>()
}

fn get_load_data_as_str(rdb: *mut raw::RedisModuleIO) -> IntervalSet {
    let data = raw::load_string(rdb).unwrap();
    return IntervalSet::from_str(&data.try_as_str().unwrap().to_string()).unwrap();
}

///Retrieving a list of sets based on CLI input.
fn get_sets<A: NextArg>(mut args: A) -> Result<Sets, RedisError> {
    let mut sets: Sets = Sets(vec![]);
    while let Ok(member) = args.next_string() {
        let set = Set {
            member,
            // If the user supplied a member, they must provide scores as well:
            min_score: args.next_i64()?,
            max_score: args.next_i64()?,
        };
        sets.0.push(set);
    }

    Ok(sets)
}

///Retrieving a list of set members based on CLI input.
fn get_members<A: NextArg>(mut args: A) -> Result<Vec<String>, RedisError> {
    let mut members = vec![];

    while let Ok(member) = args.next_string() {
        members.push(member);
    }

    Ok(members)
}

///Checking if set is in given score range.
fn is_in_score_range(set: &&Set, score: i64) -> bool {
    if set.min_score <= score && set.max_score >= score {
        return true;
    }
    return false;
}

/// Adding a new interval set.
/// This function is used for the iset.add command.
fn is_add(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key_name_arg = args.next_arg()?;

    let sets = get_sets(&mut args)?;
    if sets.0.is_empty() {
        return Err(RedisError::WrongArity);
    }

    let key = ctx.open_key_writable(&key_name_arg);
    match key.get_value::<IntervalSet>(&REDIS_INTERVAL_SETS)? {
        Some(value) => {
            println!("[iset.add] Updating key '{}'", key_name_arg);
            for item in sets.0 {
                let does_set_exist = does_set_exist(value.sets.0.clone(), item.member.to_string());
                let index = find_set(value.sets.0.clone(), item.member.to_string());

                //If set by the given name exists
                if does_set_exist == true {
                    value.sets.0[index] = Set {
                        member: item.member,
                        min_score: item.min_score,
                        max_score: item.max_score
                    }
                }
                else {
                    value.sets.0.push(item);
                }
            }
        }
        None => {
            println!("[iset.add] Adding a new key '{}'", key_name_arg);
            let value = IntervalSet { sets };
            key.set_value(&REDIS_INTERVAL_SETS, value)?;
        }
    }
    

    REDIS_OK
}

/// Checking if a key set exists in the list of sets
/// # Arguments
/// 
/// * `sets` - A list of sets
/// * `set_name` - A given set name to look for
fn does_set_exist(sets: Vec<Set>, set_name: String) -> bool {
    return sets
        .iter()
        .find(|set| set.member == set_name.to_string()) != None
}

/// Looking for a set index inside a list of given sets
/// # Arguments
/// 
/// * `sets` - A list of sets
/// * `set_name` - A given set name to look for
fn find_set(sets: Vec<Set>, set_name: String) -> usize {
    match sets
        .iter()
        .position(|set| set.member == set_name.to_string()) {
            Some(v) => {
                return v;
            }
            None => {
                return usize::MAX;
            }
        }
    
}

/// Deleting a interval set.
/// This function is used for the iset.del command.
fn is_del(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key_name_arg = args.next_arg()?;
    let members = get_members(&mut args)?;
    let key = ctx.open_key_writable(&key_name_arg);
    return match key.get_value::<IntervalSet>(&REDIS_INTERVAL_SETS)? {
        Some(value) => {
            println!("[iset.del] Deleting key '{}'", key_name_arg);
            if members.is_empty() {
                key.delete()?;
                return REDIS_OK;
            }
            else {
                for member in &members {
                    let sets: Vec<_> = value
                        .sets
                        .0
                        .iter()
                        .filter(|set| set.member == member.to_string())
                        .map(|set| {
                            vec![
                                set.member.clone().to_string(),
                            ]
                        })
                        .collect();
                    if sets.is_empty() {
                        return Err(RedisError::String(error_cannot_find_iset_member(member)));
                    }
                }
            }
            
            for member in members {
                value.sets.0.retain(|set| set.member != member)
            }
            return REDIS_OK;
        }
        None => Err(RedisError::String(error_cannot_find_iset_key(key_name_arg.try_as_str().unwrap()))).into(),
    };
}

/// Retrieving interval set info.
/// This function is used for the iset.get command.
fn is_get(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key_name_arg = args.next_arg()?;
    let key = ctx.open_key(&key_name_arg);
    match key.get_value::<IntervalSet>(&REDIS_INTERVAL_SETS)? {
        Some(value) => {
            println!("[is.get] Retrieving key '{}'", key_name_arg);
            if let Ok(member) = args.next_arg() {
                println!("[is.get] Retrieving key '{}' members '{}'", key_name_arg, member);
                let sets: Vec<_> = value
                    .sets
                    .0
                    .iter()
                    .filter(|set| set.member == member.to_string())
                    .map(|set| {
                        vec![
                            set.member.clone().to_string(),
                            set.min_score.clone().to_string(),
                            set.max_score.clone().to_string(),
                        ]
                    })
                    .collect();
                if sets.is_empty() {
                    return Err(RedisError::String(error_cannot_find_iset_member(member.try_as_str().unwrap())))
                }
                return Ok(sets.into());
            } else {
                let sets: Vec<_> = value
                    .sets
                    .0
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
        None => Err(RedisError::String(error_cannot_find_iset_key(key_name_arg.try_as_str().unwrap()))).into(),
    }
}

/// Searching for set in score range.
/// This function is used for the iset.score command.
fn is_score(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key_name_arg = args.next_arg()?;
    let score = args.next_i64()?;
    let key = ctx.open_key(&key_name_arg);
    println!("[iset.score] Retrieving key '{}' sets in score '{}'", key_name_arg, score);
    return match key.get_value::<IntervalSet>(&REDIS_INTERVAL_SETS)? {
        Some(value) => {
            let sets: Vec<_> = value
                .sets
                .0
                .iter()
                .filter(|set| is_in_score_range(set, score) == true)
                .map(|set| set.member.clone())
                .collect();
            Ok(sets.into())
        }
        None => Ok(().into()),
    };
}

/// Searching for set not in score range.
/// This function is used for the iset.not_score command.
fn is_not_score(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key_name_arg = args.next_arg()?;
    let score = args.next_i64()?;
    let key = ctx.open_key(&key_name_arg);

    println!("[iset.not_score] Retrieving key '{}' sets not in score '{}'", key_name_arg, score);
    return match key.get_value::<IntervalSet>(&REDIS_INTERVAL_SETS)? {
        Some(value) => {
            let sets: Vec<_> = value
                .sets
                .0
                .iter()
                .filter(|set| is_in_score_range(set, score) == false)
                .map(|set| set.member.clone())
                .collect();
            Ok(sets.into())
        }
        None => Ok(().into()),
    };
}

pub fn error_cannot_find_iset_key(key_name: &str) -> String {
    return format!("ERR Interval Set '{key_name}' does not exist!", key_name = key_name);
}

pub fn error_cannot_find_iset_member(member_name: &str) -> String {
    return format!("ERR Interval Set member '{member_name}' does not exist!", member_name = member_name)
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
/* 
#[test]
fn get_sets_empty() {
    let args = vec![];
    let sets = get_sets(args.into_iter());
    let sets = sets.expect("no sets");
    assert_eq!(sets.0, vec![]);
}

#[test]
fn get_sets_partial1() {
    let ctx = Context::dummy().ctx;
    let args = vec![RedisString::create(ctx, "member1")];
    let mut args = args.into_iter();
    let sets = get_sets(&mut args);
    match sets.expect_err("should fail on partial arguments") {
        RedisError::WrongArity => {}
        _ => panic!("wrong error"),
    }
}

#[test]
fn get_sets_partial2() {
    let ctx = Context::dummy().ctx;
    let args = vec![RedisString::create(ctx, "member1"), RedisString::create(ctx, "10")];
    let sets = get_sets(args.into_iter());
    match sets.expect_err("should fail on partial arguments") {
        RedisError::WrongArity => {}
        _ => panic!("wrong error"),
    }
}

#[test]
fn get_sets_single() {
    let ctx = Context::dummy().ctx;
    let args = vec![RedisString::create(ctx, "member1"), RedisString::create(ctx, "10"), RedisString::create(ctx, "20")];
    let sets = get_sets(args.into_iter());
    let sets = sets.expect("one member");
    assert_eq!(
        sets.0,
        vec![Set {
            member: "member1".to_string(),
            min_score: 10,
            max_score: 20,
        }]
    );
}

#[test]
fn get_sets_multi() {
    let ctx = Context::dummy().ctx;
    let args = vec![
        RedisString::create(ctx, "member1"),
        RedisString::create(ctx, "10"),
        RedisString::create(ctx, "20"),
        RedisString::create(ctx, "member2"),
        RedisString::create(ctx, "30"),
        RedisString::create(ctx, "40"),
    ];
    let sets = get_sets(args.into_iter());
    let sets = sets.expect("multiple members");
    assert_eq!(
        sets.0,
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
    let ctx = Context::dummy().ctx;
    let args = vec![RedisString::create(ctx, "member1")];
    let members = get_members(args.into_iter());
    let members = members.expect("one member");
    assert_eq!(
        members,
        vec!["member1"]
    );
}

#[test]
fn get_members_multi() {
    let ctx = Context::dummy().ctx;
    let args = vec![
        RedisString::create(ctx, "member1"),
        RedisString::create(ctx, "member2"),
    ];
    let members = get_members(args.into_iter());
    let members = members.expect("multiple members");
    assert_eq!(
        members,
        vec!["member1".to_string(), "member2".to_string()]
    );
}*/