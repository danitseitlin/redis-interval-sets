//use std::convert::TryInto;
//use std::ops::Try;


use crate::utils::{get_redis_connection, start_redis_server_with_module};
use anyhow::Context;
use anyhow::Result;
use redis::RedisError;
mod utils;
use redis::Connection;

static REDIS_SERVER_PORT: u16 = 6479;
static CANNOT_FIND_INTERVAL_SET_ERR: &str = "An error was signalled by the server: Interval Set does not exist!";
/*
#[test]
fn start_server() -> Result<()> {
    let _guards = vec![start_redis_server_with_module("intervalsets", REDIS_SERVER_PORT)
        .with_context(|| "failed to start redis server")?];
    Ok(())
}*/

#[test]
fn test_add_interval_set_with_single_set() -> Result<()> {
    let _guards = vec![start_redis_server_with_module("intervalsets", REDIS_SERVER_PORT)
        .with_context(|| "failed to start redis server")?];
    let mut con =
        get_redis_connection(REDIS_SERVER_PORT).with_context(|| "failed to connect to redis server")?;

    let sets = vec![
        "member1".to_string(), "10".to_string(), "12".to_string()
    ];
    let key_name = "key-single-set";
    add_interval_set(&mut con, key_name.to_string(), &sets);
    let res = get_interval_set(&mut con, key_name.to_string(), vec![]).unwrap();
    let expected: Vec<Vec<String>> = vec![sets];
    assert_eq!(
        res,
        expected
    );
    Ok(())
}

#[test]
fn test_add_interval_set_with_multi_set() -> Result<()> {
    use redis_module::RedisValue;
    let key_name = "key-multi-set";
    let _guards = vec![start_redis_server_with_module("intervalsets", REDIS_SERVER_PORT)
        .with_context(|| "failed to start redis server")?];
    let mut con =
        get_redis_connection(REDIS_SERVER_PORT).with_context(|| "failed to connect to redis server")?;
        
    let set1 = vec!["member1".to_string(), "10".to_string(), "12".to_string()];
    let set2 = vec!["member2".to_string(), "13".to_string(), "15".to_string()];
    let multisets = vec![
        "member1".to_string(), "10".to_string(), "12".to_string(),
        "member2".to_string(), "13".to_string(), "15".to_string()
    ];
    // Call the tested command
    add_interval_set(&mut con, key_name.to_string(), &multisets);
    let res = get_interval_set(&mut con, key_name.to_string(), vec![]).unwrap();
    let expected: Vec<Vec<String>> = vec![
        set1,
        set2
    ];
    assert_eq!(
        res,
        expected
    );
    Ok(())
}

#[test]
fn test_add_interval_set_with_set_and_add_more_sets() -> Result<()> {
    use redis_module::RedisValue;
    let key_name = "key-sets";
    let _guards = vec![start_redis_server_with_module("intervalsets", REDIS_SERVER_PORT)
        .with_context(|| "failed to start redis server")?];
    let mut con =
        get_redis_connection(REDIS_SERVER_PORT).with_context(|| "failed to connect to redis server")?;

    let set1 = vec!["member1".to_string(), "5".to_string(), "7".to_string()];
    let set2 = vec!["member2".to_string(), "10".to_string(), "12".to_string()];
    let set3 = vec!["member3".to_string(), "13".to_string(), "15".to_string()];
    let multisets = vec![
        "member2".to_string(), "10".to_string(), "12".to_string(),
        "member3".to_string(), "13".to_string(), "15".to_string()
    ];

    //Adding 1 set
    add_interval_set(&mut con, key_name.to_string(), &set1);
    let res = get_interval_set(&mut con, key_name.to_string(), vec![]).unwrap();
    let mut expected: Vec<Vec<String>> = vec![set1.to_owned()];
    assert_eq!(
        res,
        expected,
        "Verifying first set addition"
    );
    //Adding 2 more sets
    add_interval_set(&mut con, key_name.to_string(), &multisets);
    let res = get_interval_set(&mut con, key_name.to_string(), vec![]).unwrap();
    expected = vec![
        set1.to_owned(),
        set2,
        set3
    ];
    assert_eq!(
        res,
        expected,
        "Verifying 2 sets addition"
    );
    Ok(())
}

#[test]
fn get_non_existent_iset() -> Result<()> {
    use redis_module::RedisError;
    let key_name = "no-key";
    let _guards = vec![start_redis_server_with_module("intervalsets", REDIS_SERVER_PORT)
        .with_context(|| "failed to start redis server")?];
    let mut con =
        get_redis_connection(REDIS_SERVER_PORT).with_context(|| "failed to connect to redis server")?;
    let error = get_interval_set(&mut con, key_name.to_string(), vec![]).unwrap_err().to_string();
    assert_eq!(
        error,
        CANNOT_FIND_INTERVAL_SET_ERR.to_string()
    );
    Ok(())
}

#[test]
fn del_non_existent_iset() -> Result<()> {
    use redis_module::RedisError;
    let key_name = "no-key";
    let _guards = vec![start_redis_server_with_module("intervalsets", REDIS_SERVER_PORT)
        .with_context(|| "failed to start redis server")?];
    let mut con =
        get_redis_connection(REDIS_SERVER_PORT).with_context(|| "failed to connect to redis server")?;
    let error = del_interval_set(&mut con, key_name.to_string(), vec![]).unwrap_err().to_string();
    assert_eq!(
        error,
        CANNOT_FIND_INTERVAL_SET_ERR.to_string()
    );
    Ok(())
}
fn add_interval_set(con: &mut Connection, key_name: String, sets: &Vec<String>) {
    use redis_module::RedisValue;
    let result: Result<String, RedisError> = redis::cmd("iset.add")
        .arg(key_name)
        .arg(sets)
        .query(con);

    assert_eq!(RedisValue::SimpleString("OK".to_string()), RedisValue::SimpleString(result.unwrap()));
}

fn get_interval_set(con: &mut Connection, key_name: String, members: Vec<String>) -> Result<Vec<Vec<String>>, RedisError> {
    let res: Result<Vec<Vec<String>>, RedisError> = redis::cmd("iset.get").arg(key_name).arg(&members).query(con);
    return res;
}

fn del_interval_set(con: &mut Connection, key_name: String, members: Vec<String>) -> Result<Vec<Vec<String>>, RedisError> {
    let res: Result<Vec<Vec<String>>, RedisError> = redis::cmd("iset.del").arg(key_name).arg(&members).query(con);
    return res;
}
/***
 * is.add 

    create an iset with 1 set- DONE
    create an iset with multiple sets- DONE
    create an iset with 1 set + add more sets to it- DONE
    attemp to create iset with 1 set + add a set with the same name

is.get

    get a non existing iset- DONE
    get an existent iset- DONE
    get specific member of iset

is.del

    delete an existent set
    delete a non existence set
    delete 1 member

is.score

    filter a score with no sets found
    filter a score with 1 set found
    filter a score with 3 sets found

is.not_score

    filter a score with no sets found
    filter a score with 1 set found
    filter a score with 3 sets found

 * 
 * 
 */

