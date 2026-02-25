//! Initialization logic for earn-quest contract

use soroban_sdk::{Env, Address};
use crate::storage::{set_admin, set_version, set_config, is_initialized, mark_initialized};

pub struct InitConfig {
    pub admin: Address,
    pub version: u32,
    pub config_params: Vec<(String, String)>,
}

pub fn initialize(env: &Env, config: InitConfig) {
    if is_initialized(env) {
        panic!("Contract already initialized");
    }
    set_admin(env, &config.admin);
    set_version(env, config.version);
    set_config(env, &config.config_params);
    mark_initialized(env);
}

pub fn upgrade_authorize(env: &Env, caller: &Address) -> bool {
    let admin = crate::storage::get_admin(env);
    caller == &admin
}
