use crate::utils::{is_okay, is_not_score};
use crate::utils::{
    get_redis_connection, start_redis_server_with_module,
    add_interval_set, get_interval_set, del_interval_set,
    error_cannot_find_iset_key, error_cannot_find_iset_member
};
use anyhow::Context;
use anyhow::Result;
use utils::is_score;
mod utils;
static REDIS_SERVER_PORT: u16 = 6379;

#[test]
fn test_add_interval_set_with_single_set() -> Result<()> {
    let port: u16 = REDIS_SERVER_PORT + 1;
    let mut _guards = vec![start_redis_server_with_module("intervalsets", port)
        .with_context(|| "failed to start redis server")?];
    let mut con =
        get_redis_connection(port).with_context(|| "failed to connect to redis server")?;

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
    let port: u16 = REDIS_SERVER_PORT + 2;
    let key_name = "key-multi-set";
    let mut _guards = vec![start_redis_server_with_module("intervalsets", port)
        .with_context(|| "failed to start redis server")?];
    let mut con =
        get_redis_connection(port).with_context(|| "failed to connect to redis server")?;
        
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
    let port: u16 = REDIS_SERVER_PORT + 3;
    let key_name = "key-sets";
    let mut _guards = vec![start_redis_server_with_module("intervalsets", port)
        .with_context(|| "failed to start redis server")?];
    let mut con =
        get_redis_connection(port).with_context(|| "failed to connect to redis server")?;

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
    let port: u16 = REDIS_SERVER_PORT + 4;
    let key_name = "no-key";
    let mut _guards = vec![start_redis_server_with_module("intervalsets", port)
        .with_context(|| "failed to start redis server")?];
    let mut con =
        get_redis_connection(port).with_context(|| "failed to connect to redis server")?;
    let error = get_interval_set(&mut con, key_name.to_string(), vec![]).unwrap_err().to_string();
    assert_eq!(
        error,
        error_cannot_find_iset_key(key_name)
    );
    Ok(())
}

#[test]
fn get_existent_and_non_existent_iset_member() -> Result<()> {
    let port: u16 = REDIS_SERVER_PORT + 5;
    let key_name = "key7";
    let mut _guards = vec![start_redis_server_with_module("intervalsets", port)
        .with_context(|| "failed to start redis server")?];
    let mut con =
        get_redis_connection(port).with_context(|| "failed to connect to redis server")?;
        
    let set1 = vec!["member1".to_string(), "10".to_string(), "12".to_string()];
    let multisets = vec![
        "member1".to_string(), "10".to_string(), "12".to_string(),
        "member2".to_string(), "13".to_string(), "15".to_string()
    ];
    // Call the tested command
    add_interval_set(&mut con, key_name.to_string(), &multisets);
    let member1 = get_interval_set(&mut con, key_name.to_string(), vec!["member1".to_string()]);
    assert_eq!(
        member1.unwrap(),
        vec![set1]
    );

    let error = get_interval_set(&mut con, key_name.to_string(), vec!["member".to_string()]);
    assert_eq!(
        error.unwrap_err().to_string(),
        error_cannot_find_iset_member("member")
    );
    Ok(())
}
#[test]
fn del_non_existent_iset() -> Result<()> {
    let port: u16 = REDIS_SERVER_PORT + 6;
    let key_name = "no-key";
    let mut _guards = vec![start_redis_server_with_module("intervalsets", port)
        .with_context(|| "failed to start redis server")?];
    let mut con =
        get_redis_connection(port).with_context(|| "failed to connect to redis server")?;
    let error = del_interval_set(&mut con, key_name.to_string(), vec![]).unwrap_err().to_string();
    assert_eq!(
        error,
        error_cannot_find_iset_key(key_name)
    );
    Ok(())
}

#[test]
fn del_existent_iset() -> Result<()> {
    let port: u16 = REDIS_SERVER_PORT + 7;
    let key_name = "to-be-del-set";
    let mut _guards = vec![start_redis_server_with_module("intervalsets", port)
        .with_context(|| "failed to start redis server")?];
    let mut con =
        get_redis_connection(port).with_context(|| "failed to connect to redis server")?;
    let set3 = vec!["member3".to_string(), "13".to_string(), "15".to_string()];
    let multisets = vec![
        "member2".to_string(), "10".to_string(), "12".to_string(),
        "member3".to_string(), "13".to_string(), "15".to_string()
    ];
    add_interval_set(&mut con, key_name.to_string(), &multisets);
    let res1 = del_interval_set(&mut con, key_name.to_string(), vec!["member2".to_string()]).unwrap();
    is_okay(res1);
    let res2 = get_interval_set(&mut con, key_name.to_string(), vec![]);
    assert_eq!(
        res2.unwrap(),
        vec![
            set3
        ]
    );
    let res3 = del_interval_set(&mut con, key_name.to_string(), vec!["member1".to_string()]);
    assert_eq!(
        res3.unwrap_err().to_string(),
        error_cannot_find_iset_member("member1")
    );
    let res4 = del_interval_set(&mut con, key_name.to_string(), vec![]).unwrap();
    is_okay(res4);
    let res5 = get_interval_set(&mut con, key_name.to_string(), vec![]);
    assert_eq!(
        res5.unwrap_err().to_string(),
        error_cannot_find_iset_key(key_name)
    );
    Ok(())
}

#[test]
fn test_is_score() -> Result<()> {
    let port: u16 = REDIS_SERVER_PORT + 8;
    let key_name = "to-be-del-set";
    let mut _guards = vec![start_redis_server_with_module("intervalsets", port)
        .with_context(|| "failed to start redis server")?];
    let mut con =
        get_redis_connection(port).with_context(|| "failed to connect to redis server")?;
    add_interval_set(&mut con, key_name.to_string(), &vec!["member1".to_string(), "1".to_string(), "3".to_string()]);
    let score_empty_results = is_score(&mut con, key_name.to_string(), 5);
    let empty_vec: Vec<String> = vec![];
    assert_eq!(
        score_empty_results.unwrap(),
        empty_vec
    );
    let score_results = is_score(&mut con, key_name.to_string(), 2);
    assert_eq!(
        score_results.unwrap(),
        vec!["member1".to_string()]
    );
    Ok(())
}

#[test]
fn test_is_not_score() -> Result<()> {
    let port: u16 = REDIS_SERVER_PORT + 9;
    let key_name = "to-be-del-set";
    let mut _guards = vec![start_redis_server_with_module("intervalsets", port)
        .with_context(|| "failed to start redis server")?];
    let mut con =
        get_redis_connection(port).with_context(|| "failed to connect to redis server")?;
    add_interval_set(&mut con, key_name.to_string(), &vec!["member1".to_string(), "1".to_string(), "3".to_string()]);
    let score_empty_results = is_not_score(&mut con, key_name.to_string(), 2);
    let empty_vec: Vec<String> = vec![];
    assert_eq!(
        score_empty_results.unwrap(),
        empty_vec
    );
    let score_results = is_not_score(&mut con, key_name.to_string(), 6);
    assert_eq!(
        score_results.unwrap(),
        vec!["member1".to_string()]
    );
    Ok(())
}