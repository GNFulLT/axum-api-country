use crate::config::error::{ConfigError,ConfigResult};
use once_cell::sync::Lazy;
use std::env;

pub fn config() -> &'static Config {
    static SINGLETON : Lazy<Config> =  Lazy::new(||{
        Config::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL ERROR WHILE LOADING CONFIG FILE CAUSE : {ex:?}")
        })
    });

    return &SINGLETON
}

#[allow(non_snake_case)]
pub struct Config {
    pub POSTGRE_URL: String,
    pub PORT : u32,
    pub POOL_SIZE : u32,
    pub RESET_DB : bool
}

impl Config {
    pub fn load_from_env() -> ConfigResult<Config> {
        Ok(Config{
            POSTGRE_URL:get_env("POSTGRE_URL")?,
            PORT:get_env("PORT").unwrap_or("3000".to_string()).parse().unwrap_or_else(|_| {println!("=> {:<12} - PORT env variable is expected to be i32 type. Default value will be used.","INFO");3000}),
            POOL_SIZE:get_env("POOL_SIZE").unwrap_or("5".to_string()).parse().unwrap_or_else(|_| {println!("=> {:<12} - POOL_SIZE env variable is expected to be i32 type. Default value will be used.","INFO");5}),
            RESET_DB:get_env("RESET_DB").unwrap_or("true".to_string()).parse().unwrap_or_else(|_| {println!("=> {:<12} - RESET_DB env variable is expected to be boolean type. Default value will be used.","INFO");true}),

        })
    }
}

fn get_env(name:&'static str) -> ConfigResult<String> {
    env::var(name).map_err(|_| ConfigError::ConfigMissingEnv(name.to_string()))
}