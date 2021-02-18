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
use crate::redisjson::{Format, Path, RedisJSON, SetOptions, ValueIndex};

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

///
/// Backwards compatibility convertor for RedisJSON 1.x clients
///



//////////////////////////////////////////////////////

pub extern "C" fn init(raw_ctx: *mut rawmod::RedisModuleCtx) -> c_int {
    crate::commands::index::schema_map::init();
    redisearch_api::init(raw_ctx)
}

redis_module! {
    name: "RedisIntervalSets",
    version: 99_99_99,
    data_types: [
        REDIS_JSON_TYPE,
    ],
    init: init,
    commands: [
        ["iadd", json_del, "write", 1,1,1],
        ["ifilter", json_get, "readonly", 1,1,1],
        ["idel", json_mget, "write", 1,1,1]
    ],
}