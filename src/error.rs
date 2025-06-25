use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to write config to disk {source}")]
    SaveFaield {
        #[from]
        source: std::io::Error,
    },

    #[error("Failed to serialize config")]
    SerializationFailed(#[from] toml::ser::Error),

    #[error("Failed to load config")]
    ConfigError(#[from] config::ConfigError),
}
