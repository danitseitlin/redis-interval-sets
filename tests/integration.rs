//use redis::{Commands, RedisResult, RedisError};
use anyhow::Result;

#[test]
fn iset_add_single_set() -> Result<()> {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
    let res = redis::cmd("iset.add")
                .arg("single_set")
                .arg("highschool")
                .arg("12")
                .arg("18")
                .query(&mut con);
    assert_eq!(
        res,
        Ok(())
    );
    Ok(())
}

#[test]
fn iset_add_multi_set() -> Result<()> {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
    let res = redis::cmd("iset.add")
                .arg("multi_set")
                .arg("highschool")
                .arg("12")
                .arg("18")
                .arg("garden")
                .arg("1")
                .arg("7")
                .query(&mut con);
    assert_eq!(
        res,
        Ok(())
    );
    Ok(())
}

#[test]
fn iset_add_triple_set() -> Result<()> {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
    let mut res = redis::cmd("iset.add")
                .arg("tripleset")
                .arg("highschool")
                .arg("12")
                .arg("18")
                .query(&mut con);
    assert_eq!(
        res,
        Ok(())
    );
    res = redis::cmd("iset.add")
                .arg("tripleset")
                .arg("garden")
                .arg("1")
                .arg("7")
                .query(&mut con);
    assert_eq!(
        res,
        Ok(())
    );
    Ok(())
}

#[test]
fn iset_get_non_existing_set() -> Result<()> {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
    let mut res = redis::cmd("iset.get").arg("non_existing").query(&mut con);
    assert_eq!(
        res,
        Ok(())
    );
    Ok(())
}

#[test]
fn iset_get_existing_set() -> Result<()> {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
    let mut res = redis::cmd("iset.get").arg("tripleset").query(&mut con);
    assert_eq!(
        res,
        Ok(())
    );
    Ok(())
}

/*#[test]
fn iset_score_non_existent_range() -> Result<()> {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
}

#[test]
fn iset_score_one_set() -> Result<()> {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
}

#[test]
fn iset_score_three_sets() -> Result<()> {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
}

#[test]
fn iset_not_score_non_existent_range() -> Result<()> {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
}

#[test]
fn iset_not_score_one_set() -> Result<()> {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
}

#[test]
fn iset_not_score_three_sets() -> Result<()> {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
}

#[test]
fn iset_del_non_existent_set() -> Result<()> {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
}

#[test]
fn iset_del_an_existent_set() -> Result<()> {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
}

#[test]
fn iset_del_non_existent_set() -> Result<()> {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;   
}*/