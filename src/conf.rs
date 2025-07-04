use config::Environment;

use crate::error::AppError;

const FOCUS_DURATION: i32 = 1500;
const SHORT_BREAK_DURATION: i32 = 300;
const LONG_BREAK_DURATION: i32 = 900;
const SESSIONS: i32 = 4;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Config {
    pub focus_duration: i32,
    pub short_break_duration: i32,
    pub long_break_duration: i32,
    pub sessions: i32,
    pub enable_notifications: bool,
    pub critical_notifications: bool,
    pub tick_sound: bool,
}

impl Config {
    pub fn new() -> Result<Config, AppError> {
        let config_dir = Self::dir();
        let config = config::Config::builder()
            .set_default("focus_duration", FOCUS_DURATION)?
            .set_default("short_break_duration", SHORT_BREAK_DURATION)?
            .set_default("long_break_duration", LONG_BREAK_DURATION)?
            .set_default("sessions", SESSIONS)?
            .set_default("enable_notifications", true)?
            .set_default("critical_notifications", true)?
            .set_default("tick_sound", true)?
            .add_source(
                config::File::with_name(config_dir.join("config.toml").to_str().unwrap())
                    .required(false),
            )
            .add_source(Environment::with_prefix("TEMPORIS"))
            .build()?;
        config
            .try_deserialize()
            .map_err(|e| AppError::ConfigError(e))
    }
    pub fn dir() -> std::path::PathBuf {
        let cfg_path = directories::ProjectDirs::from("com", "reciperium", "temporis")
            .expect("to be able to build config directory")
            .config_dir()
            .to_path_buf();

        cfg_path
    }

    pub fn save(&mut self) -> Result<(), AppError> {
        let toml_string = toml::to_string_pretty(self)?;
        let config_dir = Self::dir();
        std::fs::create_dir_all(&config_dir)?;
        std::fs::write(config_dir.join("config.toml"), toml_string)?;
        Ok(())
    }
}
