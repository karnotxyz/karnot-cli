use bollard::models::HostConfig;
use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::utils::docker::run_docker_image;

pub fn explorer() {
    let random_string: String = (0..64).map(|_| rand::thread_rng().sample(Alphanumeric).to_string()).collect();
    let secret_key_base = format!("SECRET_KEY_BASE={}", random_string);

    let env = vec![
        "RPC_API_HOST=\"http://127.0.0.1:9944\"",
        "DB_TYPE=sqlite",
        "DISABLE_MAINNET_SYNC=true",
        "DISABLE_TESTNET_SYNC=true",
        "TESTNET_RPC_API_HOST=\"http://127.0.0.1:9944\"",
        "DATABASE_PATH=/use/exp.db",
        &secret_key_base,
    ];

    let host_config = HostConfig { network_mode: Some(String::from("host")), ..Default::default() };

    run_docker_image(
        "ghcr.io/lambdaclass/stark_compass_explorer:v0.2.34.2",
        "madara-explorer",
        Some(env),
        Some(host_config),
    );
}
