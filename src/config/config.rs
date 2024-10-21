use std::borrow::ToOwned;
use std::io::{Error, Read};
use std::path;
use std::path::Path;
use lazy_static::lazy_static;
use tracing::{info, warn};
use matrix_sdk::ruma::exports::serde::{Deserialize, Serialize};
use std::sync::RwLock;
use serde::de::Expected;
use walle_core::config::ImplConfig;
use crate::constant::MATRIX_ONEBOT;

type IOResult<T> = Result<T, Error>;

const CONFIG_PATH: &'static str = "config.toml";
const IN_INIT_LOG_NAME_CONFIG: &'static str = "config";
const IN_INIT_LOG_NAME_LANG: &'static str = "language file";

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new(
        Config::load_or_new(CONFIG_PATH, IN_INIT_LOG_NAME_CONFIG)
        .expect(&format!("Failed to load {}, exit", IN_INIT_LOG_NAME_CONFIG))
    );
    pub static ref LANG: RwLock<Lang> = RwLock::new(
        Lang::load_or_new(
            &format!("lang/{}", &*CONFIG.read().unwrap().lang_file),
            IN_INIT_LOG_NAME_LANG,
        )
        .expect(&format!("Failed to load {}, exit", IN_INIT_LOG_NAME_LANG))
    );
    // pub static ref ASYNC_CONFIG: tokio::sync::RwLock<Config> = tokio::sync::RwLock::new(
    //     Config::load_or_new(CONFIG_PATH, IN_INIT_LOG_NAME_CONFIG)
    //     .expect(&format!("Failed to load {}, exit", IN_INIT_LOG_NAME_CONFIG))
    // );
    // pub static ref ASYNC_LANG: tokio::sync::RwLock<Lang> = tokio::sync::RwLock::new(
    //     Lang::load_or_new(
    //         &format!("lang/{}", &*CONFIG.read().unwrap().lang_file),
    //         IN_INIT_LOG_NAME_LANG,
    //     )
    //     .expect(&format!("Failed to load {}, exit", IN_INIT_LOG_NAME_LANG))
    // );
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub full_user_id: String, // 用户完整ID
    pub password: String, // 用户密码
    pub server_domain: String, // 服务器域(如需单独指定连接服务器)

    pub lang_file: String, // 指定语言文件

    pub onebot: OBConfig,
    pub meta: MetaConfig,
    pub onebot_conn: ImplConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OBConfig {
    pub query_self_event_interval_secs: u64, // OneBot Actions Handler 查询自身信息间隔
    pub is_intercept_self_message: bool, // 是否拦截自身信息(不传递给应用端)

    pub is_message_forward_struct: bool, // message中是否返回数据结构 (否: 用户可读字符串)
    pub is_alt_message_forward_struct: bool, // message中是否返回数据结构 (否: 用户可读字符串)
}

impl Default for OBConfig {
    fn default() -> Self {
        Self {
            query_self_event_interval_secs: 1,
            is_intercept_self_message: true,
            is_message_forward_struct: true,
            is_alt_message_forward_struct: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MetaConfig {
    pub log_level: LogLevel,
    // pub event_cache_size: usize, // 维护数据库, 无cache
    // pub sled: bool, // 这是什么?
    // pub leveldb: bool, // 这是什么?
    // pub data_path: Option<String>, // 暂时不用
    pub log_path: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            full_user_id: Default::default(),
            password: Default::default(),
            server_domain: Default::default(),
            lang_file: "zh-cn.toml".to_owned(),
            onebot: Default::default(),
            meta: Default::default(),
            onebot_conn: Default::default(),
        }
    }
}

trait NewConfig: Sized {
    fn new_config() -> Self;
    fn ser(&self) -> IOResult<String>;
    fn de(s: &str) -> IOResult<Self>;
}

trait LoadConfig: for<'de> Deserialize<'de> + NewConfig {
    fn save_to_file(&self, path: &str) -> IOResult<()> {
        let data = self.ser()?;
        if let Err(e) = std::fs::write(path, &data) {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    let path = Path::new(path);
                    std::fs::create_dir_all(path.parent().unwrap())?;
                    std::fs::write(path, data)?;
                    Ok(())
                }
                _ => Err(e),
            }
        } else { Ok(()) }
    }
    fn load_from_file(path: &str) -> IOResult<Self> {
        let mut file = std::fs::File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        Self::de(&data)
    }
    fn load_or_new(path: &str, ty: &str) -> IOResult<Self> {
        info!(target: MATRIX_ONEBOT,"Loading {} from {}", ty, path);
        match Self::load_from_file(path) {
            Ok(config) => {
                info!(target: MATRIX_ONEBOT,"Success load {} from {}", ty, path);
                Ok(config)
            },
            Err(err) => match err.kind() {
                std::io::ErrorKind::Other => {
                    warn!(target: MATRIX_ONEBOT, "Serialize {} error {}", ty, err);
                    Err(err)
                }
                _ => {
                    warn!(target: MATRIX_ONEBOT, "Open {} failed: {}", path, err);
                    info!(target: MATRIX_ONEBOT, "Create new {}: {}", ty, path);
                    let config = Self::new_config();
                    config.save_to_file(path)?;
                    Ok(config)
                }
            }
        }
    }
}

impl NewConfig for Config {
    fn new_config() -> Self { Self::default() }
    fn ser(&self) -> IOResult<String> {
        toml::to_string(self).map_err(|e| Error::new(std::io::ErrorKind::Other, e))
    }
    fn de(s: &str) -> IOResult<Self> {
        toml::from_str(s).map_err(|e| Error::new(std::io::ErrorKind::Other, e))
    }
}

impl LoadConfig for Config {}

// Lang
// todo: 优化 - 实现 &'static str
#[derive(Serialize, Deserialize, Debug)]
pub struct Lang {
    pub error_matrix_login_failed: String,
    pub error_matrix_add_event_handler_failed: String,
    pub error_matrix_sync_failed: String,

    pub error_matrix_login_user_id_is_none: String,

    pub error_database_connection_failed: String,
    pub error_database_table_init_failed: String,
    pub error_database_table_insert_failed: String,
    pub error_database_table_query_failed: String,
    pub error_database_table_delete_failed: String,
    pub error_database_table_edit_failed: String,
    pub error_database_migrate_failed: String,
}

impl Default for Lang {
    fn default() -> Self {
        Self {
            error_matrix_login_failed: "错误: 登录失败".to_owned(),
            error_matrix_add_event_handler_failed: "错误: 事件处理器添加失败".to_owned(),
            error_matrix_sync_failed: "错误: 同步失败".to_owned(),

            error_matrix_login_user_id_is_none: "错误: 未填写登录 Matrix 的用户FullId".to_owned(),

            error_database_connection_failed: "错误: 数据库连接失败".to_owned(),
            error_database_table_init_failed: "错误: 数据库表 {table} 初始化失败".to_owned(),
            error_database_table_insert_failed: "错误: 数据库表 {table} 插入失败".to_owned(),
            error_database_table_query_failed: "错误: 数据库表 {table} 查询失败: {error}".to_owned(),
            error_database_table_delete_failed: "错误: 数据库表 {table} 删除(行)失败".to_owned(),
            error_database_table_edit_failed: "错误: 数据库表 {table} 修改失败".to_owned(),
            error_database_migrate_failed: "错误: 数据库迁移失败, 从 {ver} 版本".to_owned(),
        }
    }
}

impl NewConfig for Lang {
    fn new_config() -> Self { Self::default() }
    fn ser(&self) -> IOResult<String> {
        toml::to_string(self).map_err(|e| Error::new(std::io::ErrorKind::Other, e))
    }
    fn de(s: &str) -> IOResult<Self> {
        toml::from_str(s).map_err(|e| Error::new(std::io::ErrorKind::Other, e))
    }
}

impl LoadConfig for Lang { }