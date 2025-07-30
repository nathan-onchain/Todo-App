use dotenvy::dotenv;
use env_logger;
use log::info; // if you want to use `info!()` macros

use std::env;

pub fn load_env() {
    dotenv().ok();
    env_logger::init();
}