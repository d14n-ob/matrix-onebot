use std::borrow::ToOwned;
use std::io::{Error, Read};
use std::path;
use std::path::Path;
use lazy_static::lazy_static;
use tracing::{info, warn};
use matrix_sdk::ruma::exports::serde::{Deserialize, Serialize};
use std::sync::RwLock;
use walle_core::config::ImplConfig;
use crate::constant::MATRIX_ONEBOT;

type IOResult<T> = Result<T, Error>;

const CONFIG_PATH: &'static str = "config.toml";

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new(
        Config::load_or_new(CONFIG_PATH).expect("Failed to load config, exit")
    );
    pub static ref LANG: RwLock<Lang> = RwLock::new(
        Lang::load_or_new(
            &format!("lang/{}", &*CONFIG.read().unwrap().lang_file)
        ).
        expect("Failed to load language file, exit")
    );
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub full_user_id: String, // 用户完整ID
    pub password: String, // 用户密码
    pub server_domain: String, // 服务器域(如需单独指定连接服务器)

    pub lang_file: String, // 指定语言文件

    pub meta: MetaConfig,
    pub onebot_conn: ImplConfig,
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
    fn load_or_new(path: &str) -> IOResult<Self> {
        info!(target: MATRIX_ONEBOT,"Loading config from {}", path);
        match Self::load_from_file(path) {
            Ok(config) => {
                info!(target: MATRIX_ONEBOT,"Success load config from {}", path);
                Ok(config)
            },
            Err(err) => match err.kind() {
                std::io::ErrorKind::Other => {
                    warn!(target: MATRIX_ONEBOT, "Serialize config error {}", err);
                    Err(err)
                }
                _ => {
                    warn!(target: MATRIX_ONEBOT, "Open {} failed: {}", path, err);
                    info!(target: MATRIX_ONEBOT, "Create new config: {}", path);
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
#[derive(Serialize, Deserialize, Debug)]
pub struct Lang {
    pub error_matrix_login_user_id_is_none: String,
}

impl Default for Lang {
    fn default() -> Self {
        Self {
            error_matrix_login_user_id_is_none: "错误: 未填写登录 Matrix 的用户FullId".to_owned()
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