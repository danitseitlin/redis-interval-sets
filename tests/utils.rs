use anyhow::{Context, Result};

use redis::Connection;
use redis_module::RedisValue;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use redis::RedisError;

/// Ensure child process is killed both on normal exit and when panicking due to a failed test.
pub struct ChildGuard {
    name: &'static str,
    child: std::process::Child,
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        if let Err(e) = self.child.kill() {
            println!("Could not kill {}: {}", self.name, e);
        }
        if let Err(e) = self.child.wait() {
            println!("Could not wait for {}: {}", self.name, e);
        }
    }
}

pub fn start_redis_server_with_module(module_name: &str, port: u16) -> Result<ChildGuard> {
    let extension = if cfg!(target_os = "macos") {
        "dylib"
    } else {
        "so"
    };

    let profile = if cfg!(not(debug_assertions)) {
        "release"
    } else {
        "debug"
    };
    let module_path: PathBuf = [
        std::env::current_dir()?,
        PathBuf::from(format!(
            "target/{}/lib{}.{}",
            profile, module_name, extension
        )),
    ]
    .iter()
    .collect();

    let module_path = format!("{}", module_path.display());
    assert!(fs::metadata(&module_path)
        .with_context(|| format!("Loading redis module: {}", module_path.as_str()))?
        .is_file());
    println!("Loading redis module: {}", module_path.as_str());
    let args = &[
        "--port",
        &port.to_string(),
        "--loadmodule",
         module_path.as_str()
    ];

    let redis_server = Command::new("redis-server")
        .args(args)
        .spawn()
        .map(|c| ChildGuard {
            name: "redis-server",
            child: c,
        }).with_context(|| format!("Error in raising redis-server => {}", module_path.as_str()))?;

    Ok(redis_server)
}

// Get connection to Redis
pub fn get_redis_connection(port: u16) -> Result<Connection> {
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", port))?;
    loop {
        let res = client.get_connection();
        match res {
            Ok(con) => return Ok(con),
            Err(e) => {
                if e.is_connection_refusal() {
                    // Redis not ready yet, sleep and retry
                    std::thread::sleep(Duration::from_millis(50));
                } else {
                    return Err(e.into());
                }
            }
        }
    }
}

pub fn error_cannot_find_iset_key(key_name: &str) -> String {
    return format!("An error was signalled by the server: Interval Set '{key_name}' does not exist!", key_name = key_name);
}

pub fn error_cannot_find_iset_member(member_name: &str) -> String {
    return format!("An error was signalled by the server: Interval Set member '{member_name}' does not exist!", member_name = member_name)
}

pub fn is_okay(res: String) {
    assert_eq!(
        RedisValue::SimpleString(res),
        RedisValue::SimpleString("OK".to_string())
    )
}

pub fn add_interval_set(con: &mut Connection, key_name: String, sets: &Vec<String>) {
    let result: Result<String, RedisError> = redis::cmd("iset.add")
        .arg(key_name)
        .arg(sets)
        .query(con);
    is_okay(result.unwrap());
}

pub fn get_interval_set(con: &mut Connection, key_name: String, members: Vec<String>) -> Result<Vec<Vec<String>>, RedisError> {
    let res: Result<Vec<Vec<String>>, RedisError> = redis::cmd("iset.get").arg(key_name).arg(&members).query(con);
    return res;
}

pub fn del_interval_set(con: &mut Connection, key_name: String, members: Vec<String>) -> Result<String, RedisError> {
    let res: Result<String, RedisError> = redis::cmd("iset.del").arg(key_name).arg(&members).query(con);
    return res;
}

pub fn is_score(con: &mut Connection, key_name: String, value: u16) -> Result<Vec<String>, RedisError> {
    let res: Result<Vec<String>, RedisError> = redis::cmd("iset.score").arg(key_name).arg(&value).query(con);
    return res;
}

pub fn is_not_score(con: &mut Connection, key_name: String, value: u16) -> Result<Vec<String>, RedisError> {
    let res: Result<Vec<String>, RedisError> = redis::cmd("iset.not_score").arg(key_name).arg(&value).query(con);
    return res;
}