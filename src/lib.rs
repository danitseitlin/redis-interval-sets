#[macro_use]
extern crate redis_module;

use redis_module::native_types::RedisType;
use redis_module::raw::RedisModuleTypeMethods;
use redis_module::{raw as rawmod, NextArg};
use redis_module::{Context, RedisError, RedisResult, RedisValue, REDIS_OK};
use serde_json::{Number, Value};

use std::os::raw::c_int;
use std::{i64, usize};

mod array_index;
mod backward;
mod commands;
mod error;
mod formatter;
mod nodevisitor;
mod redisjson;
mod schema; // TODO: Remove

use crate::array_index::ArrayIndex;
use crate::commands::index;
use crate::error::Error;

pub const VERSION: i32 = 1;

static RIS_TYPE: RedisType = RedisType::new(
    "RedisIntervalSets",
    VERSION,
    RedisModuleTypeMethods {
        version: redis_module::TYPE_METHOD_VERSION,

        rdb_load: Some(redisjson::type_methods::rdb_load),
        rdb_save: Some(redisjson::type_methods::rdb_save),
        aof_rewrite: None, // TODO add support
        free: Some(redisjson::type_methods::free),

        // Currently unused by Redis
        mem_usage: None,
        digest: None,

        // Auxiliary data (v2)
        aux_load: Some(redisjson::type_methods::aux_load),
        aux_save: Some(redisjson::type_methods::aux_save),
        aux_save_triggers: rawmod::Aux::Before as i32,
    },
);

fn is_add() 

fn is_del()

fn is_filter()

//////////////////////////////////////////////////////

pub extern "C" fn init(raw_ctx: *mut rawmod::RedisModuleCtx) -> c_int {
    crate::commands::index::schema_map::init();
}

redis_module! {
    name: "RedisIntervalSets",
    version: 99_99_99,
    data_types: [
        RIS_TYPE,
    ],
    init: init,
    commands: [
        ["is.add", is_add, "write", 1,1,1],
        ["is.filter", is_filter, "readonly", 1,1,1],
        ["is.del", is_del, "write", 1,1,1]
    ],
}
