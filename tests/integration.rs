use redis::{Commands, RedisResult, RedisError};
use anyhow::Result;
use redis_module::{raw, Context, NextArg, /*RedisError, RedisResult,*/ REDIS_OK};

#[test]
fn iset_add_single_set() -> RedisResult {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
    let res: RedisResult = redis::cmd("iset.add")
                .arg("single_set")
                .arg("highschool")
                .arg("12")
                .arg("18")
                .query(&mut con);
    //println!("{:?}", res.ToString());
    /*assert_eq!(
        res,
        REDIS_OK
    );*/
    REDIS_OK
}

#[test]
fn iset_add_multi_set() -> RedisResult {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
    let res: RedisResult = redis::cmd("iset.add")
                .arg("multi_set")
                .arg("highschool")
                .arg("12")
                .arg("18")
                .arg("garden")
                .arg("1")
                .arg("7")
                .query(&mut con);
    
    //println!("{:?}", res.ToString());
    /*assert_eq!(
        res,
        REDIS_OK
    );*/
    REDIS_OK
}

#[test]
fn iset_add_triple_set() -> RedisResult {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
    let mut res: RedisResult = redis::cmd("iset.add")
                .arg("tripleset")
                .arg("highschool")
                .arg("12")
                .arg("18")
                .query(&mut con);
    
    //println!("{:?}", res.ToString());
    /*assert_eq!(
        res,
        REDIS_OK
    );*/
    res = redis::cmd("iset.add")
                .arg("tripleset")
                .arg("garden")
                .arg("1")
                .arg("7")
                .query(&mut con);
    
    //println!("{:?}", res.ToString());
    /*assert_eq!(
        res,
        REDIS_OK
    );*/
    res = redis::cmd("iset.add")
                .arg("tripleset")
                .arg("gardenX")
                .arg("4")
                .arg("6")
                .query(&mut con);
    
    //println!("{:?}", res.ToString());
    /*assert_eq!(
        res,
        REDIS_OK
    );*/
    res = redis::cmd("iset.add")
                .arg("tripleset")
                .arg("gardenY")
                .arg("3")
                .arg("6")
                .query(&mut con);
    
    //println!("{:?}", res.ToString());
    /*assert_eq!(
        res,
        REDIS_OK
    );*/
    REDIS_OK
}

#[test]
fn iset_get_non_existing_set() -> RedisResult {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
    let res: RedisResult = redis::cmd("iset.get").arg("non_existing").query(&mut con);
    
    //println!("{:?}", res.ToString());
    /*assert_eq!(
        res,
        vec![]
    );*/
    REDIS_OK
}

#[test]
fn iset_get_existing_set() -> RedisResult {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
    let res: RedisResult = redis::cmd("iset.get").arg("tripleset").query(&mut con);
    
    //println!("{:?}", res.ToString());
    /*assert_eq!(
        res,
        vec![Set {
            member: "highschool".ToString(),
            min_score: 12,
            max_score: 18,
        }, Set {
            member: "garden".ToString(),
            min_score: 1,
            max_score: 7,
        }, Set {
            member: "gardenX".ToString(),
            min_score: 4,
            max_score: 6,
        }, Set {
            member: "gardenY".ToString(),
            min_score: 3,
            max_score: 6,
        }]
    );*/
    REDIS_OK
}

#[test]
fn iset_score_non_existent_range() -> RedisResult {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
    let res: RedisResult = redis::cmd("iset.score").arg("tripleset").arg("101").query(&mut con);
    
    //println!("{:?}", res.ToString());
    /*assert_eq!(
        res,
        vec![]
    );*/
    REDIS_OK
}

#[test]
fn iset_score_one_set() -> RedisResult {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
    let res: RedisResult = redis::cmd("iset.score").arg("tripleset").arg("2").query(&mut con);
    
    //println!("{:?}", res.ToString());
    /*assert_eq!(
        res,
        vec![Set {
            member: "garden".ToString(),
            min_score: 1,
            max_score: 7,
        }]
    );*/
    REDIS_OK
}

#[test]
fn iset_score_three_sets() -> RedisResult {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
    let res: RedisResult = redis::cmd("iset.score").arg("tripleset").arg("5").query(&mut con);
    
    //println!("{:?}", res.ToString());
    /*assert_eq!(
        res,
        vec![Set {
            member: "garden".ToString(),
            min_score: 1,
            max_score: 7,
        }, Set {
            member: "gardenX".ToString(),
            min_score: 4,
            max_score: 6,
        }, Set {
            member: "gardenY".ToString(),
            min_score: 3,
            max_score: 6,
        }]
    );*/
    REDIS_OK
}

#[test]
fn iset_not_score_non_existent_range() -> RedisResult {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
    let res: RedisResult = redis::cmd("iset.get").arg("not_score").arg("tripleset").arg("5").query(&mut con);
    
    //println!("{:?}", res.ToString());
    /*assert_eq!(
        res,
        vec![]
    );*/
    REDIS_OK
}

#[test]
fn iset_not_score_one_set() -> RedisResult {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
    let res: RedisResult = redis::cmd("iset.get").arg("not_score").arg("tripleset").arg("3").query(&mut con);
    
    //println!("{:?}", res.ToString());
    /*assert_eq!(
        res,
        vec![Set {
            member: "highschool".ToString(),
            min_score: 12,
            max_score: 18,
        }]
    );*/
    REDIS_OK
}

#[test]
fn iset_not_score_three_sets() -> RedisResult {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
    let res: RedisResult = redis::cmd("iset.get").arg("not_score").arg("tripleset").arg("12").query(&mut con);
    
    //println!("{:?}", res.ToString());
    /*assert_eq!(
        res,
        vec![Set {
            member: "garden".ToString(),
            min_score: 1,
            max_score: 7,
        }, Set {
            member: "gardenX".ToString(),
            min_score: 4,
            max_score: 6,
        }, Set {
            member: "gardenY".ToString(),
            min_score: 3,
            max_score: 6,
        }]
    );*/
    REDIS_OK
}

#[test]
fn iset_del_non_existent_set() -> RedisResult {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
    let res: RedisResult = redis::cmd("iset.del").arg("XSET").query(&mut con);
    
    //println!("{:?}", res.ToString());
    /*assert_eq!(
        res,
        Ok(("ERROR"))
    );*/
    REDIS_OK
}

#[test]
fn iset_del_an_existent_set() -> RedisResult {
    // Connect to Redis
    let client = redis::Client::open(format!("redis://127.0.0.1:{}/", "6379"))?;
    let mut con = client.get_connection()?;
    let res: RedisResult = redis::cmd("iset.del").arg("tripleset").query(&mut con);
    
    //println!("{:?}", res.ToString());
    /*assert_eq!(
        res,
        REDIS_OK
    );*/
    REDIS_OK
}