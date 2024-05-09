pub type ConfigResult<T> = core::result::Result<T,ConfigError>;

#[derive(Debug)]
pub enum ConfigError {
    ConfigMissingEnv(String),
}