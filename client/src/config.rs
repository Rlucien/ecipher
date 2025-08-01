
use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[cfg(feature = "database")]
pub use conf::db_path;

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
use platform_dirs::AppDirs;

use uuid::Uuid;
use once_cell::sync::Lazy;
use std::{fs, path::PathBuf, sync::Mutex};
use anyhow::{Context, Result};
use log::debug;

const CARGO_TOML: &str = include_str!("../../Cargo.toml");
static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(Config::default()));


pub fn init() {
    if let Err(e) = CONFIG.lock().unwrap().init() {
        log::error!("{e:?}");
        panic!("{:?}", e);
    }
}




#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    #[serde(skip)]
    pub config_path: PathBuf,

    #[serde(skip)]
    pub db_path: PathBuf,

    #[serde(skip)]
    pub cache_dir: PathBuf,

    #[serde(skip)]
    pub is_first_run: bool,

    #[serde(skip)]
    pub app_name: String,

    #[serde(default = "appid_default")]
    pub appid: String,

    // pub preference: Preference,

    // pub proxy: Proxy,
}

impl Config {
    pub fn init(&mut self) -> Result<()> {
        let metadata = toml::from_str::<toml::Table>(CARGO_TOML).expect("Parse Cargo.toml error");

        self.app_name = metadata
            .get("package").unwrap()
            .get("name").unwrap()
            .to_string()
            .trim_matches('"')
            .to_string();

        let pkg_name = if cfg!(any(target_os = "windows",target_os = "linux",target_os = "macos")) {
            self.app_name.clone()
        }
        else {
            metadata
                .get("package").unwrap()
                .get("metadata").unwrap()
                .get("android").unwrap()
                .get("package").unwrap()
                .to_string()
                .trim_matches('"')
                .to_string()
        };

        let app_dirs = AppDirs::new(Some(&pkg_name), true).unwrap();
        self.init_config(&app_dirs)?;
        self.load().with_context(|| "load config file failed")?;
        debug!("{:?}", self);
        Ok(())
    }

    fn init_config(&mut self, app_dirs: &AppDirs) -> Result<()> {
        self.db_path = app_dirs.data_dir.join(format!("{}.db", self.app_name));
        self.config_path = app_dirs.config_dir.join(format!("{}.toml", self.app_name));
        self.cache_dir = app_dirs.data_dir.join("cache");

        if self.appid.is_empty() {
            self.appid = appid_default();
        }

        fs::create_dir_all(&app_dirs.data_dir)?;
        fs::create_dir_all(&app_dirs.config_dir)?;
        fs::create_dir_all(&self.cache_dir)?;

        Ok(())
    }

    fn load(&mut self) -> Result<()> {
        match fs::read_to_string(&self.config_path) {
            Ok(text) => match toml::from_str::<Config>(&text) {
                Ok(c) => {
                    self.appid = c.appid;
                    // self.preference = c.preference;
                    // self.proxy = c.proxy;
                    Ok(())
                }
                Err(_) => {
                    self.is_first_run = true;

                    if let Some(bak_file) = &self.config_path.as_os_str().to_str() {
                        let bak_file = format!("{}.bak", bak_file);
                        _ = fs::copy(&self.config_path, &bak_file);
                    }

                    match toml::to_string_pretty(self) {
                        Ok(text) => Ok(fs::write(&self.config_path, text)?),
                        Err(e) => Err(e.into()),
                    }
                }
            },
            Err(_) => {
                self.is_first_run = true;

                if let Some(bak_file) = &self.config_path.as_os_str().to_str() {
                    let bak_file = format!("{}.bak", bak_file);
                    _ = fs::copy(&self.config_path, &bak_file);
                }

                match toml::to_string_pretty(self) {
                    Ok(text) => Ok(fs::write(&self.config_path, text)?),
                    Err(e) => Err(e.into()),
                }
            }
        }
    }

    pub fn save(&self) -> Result<()> {
        match toml::to_string_pretty(self) {
            Ok(text) => Ok(fs::write(&self.config_path, text)
                .with_context(|| "save config failed".to_string())?),
            Err(e) => anyhow::bail!(format!("convert config from toml format failed. {e:?}")),
        }
    }
}



pub fn appid_default() -> String {
    Uuid::new_v4().to_string()
}