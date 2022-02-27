//use std::convert::TryInto;
//use std::ops::Try;

use crate::utils::{get_redis_connection, start_redis_server_with_module};
use anyhow::Context;
use anyhow::Result;
use redis::RedisError;
use redis_module::RedisResult;
mod utils;

#[test]
fn get_sets_partial1() -> Result<()> {
    use redis_module::RedisValue;
    let port: u16 = 6479;
    let _guards = vec![start_redis_server_with_module("intervalsets", port)
        .with_context(|| "failed to start redis server")?];
    let mut con =
        get_redis_connection(port).with_context(|| "failed to connect to redis server")?;

    // Call the tested command
    let result: Result<String, RedisError> = redis::cmd("iset.add")
        .arg(&["key1", "member1"])
        .arg(10)
        .arg(12)
        .query(&mut con);
        
    assert_eq!(RedisValue::SimpleString("OK".to_string()), RedisValue::SimpleString(result.unwrap()));
    let sets: Result<Vec<Vec<String>>, RedisError> = redis::cmd("iset.get").arg(&["key1"]).query(&mut con);
    let expected: Vec<Vec<String>> = vec![vec![
        "member1".to_string(),
        "10".to_string(),
        "12".to_string()
    ]];
    assert_eq!(
        sets.unwrap(),
        expected
    );
    Ok(())
}