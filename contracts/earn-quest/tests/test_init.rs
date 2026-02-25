//! Tests for contract initialization and upgradability

use soroban_sdk::{testutils::Address as TestAddress, Env, Address, Vec};
use crate::{init, storage};

#[test]
fn test_initialize_sets_admin_and_version() {
    let env = Env::default();
    let admin = TestAddress::random(&env);
    let config_params = Vec::new(&env);
    let version = 1;
    let config = init::InitConfig {
        admin: admin.clone(),
        version,
        config_params: config_params.clone(),
    };
    init::initialize(&env, config);
    assert_eq!(storage::get_admin(&env), admin);
    assert_eq!(storage::get_version(&env), version);
    assert_eq!(storage::get_config(&env), config_params);
    assert!(storage::is_initialized(&env));
}

#[test]
#[should_panic]
fn test_reinitialization_panics() {
    let env = Env::default();
    let admin = TestAddress::random(&env);
    let config_params = Vec::new(&env);
    let version = 1;
    let config = init::InitConfig {
        admin: admin.clone(),
        version,
        config_params: config_params.clone(),
    };
    init::initialize(&env, config.clone());
    // Second call should panic
    init::initialize(&env, config);
}

#[test]
fn test_upgrade_authorize_only_admin() {
    let env = Env::default();
    let admin = TestAddress::random(&env);
    let config_params = Vec::new(&env);
    let version = 1;
    let config = init::InitConfig {
        admin: admin.clone(),
        version,
        config_params: config_params.clone(),
    };
    init::initialize(&env, config);
    let not_admin = TestAddress::random(&env);
    assert!(init::upgrade_authorize(&env, &admin));
    assert!(!init::upgrade_authorize(&env, &not_admin));
}
