use bollard::models::{HostConfig,PortBinding};
use rand::distributions::Alphanumeric;
use rand::Rng;

use std::collections::HashMap;

use crate::utils::docker::run_docker_image;

pub fn explorer() {
    let random_string: String = (0..64).map(|_| rand::thread_rng().sample(Alphanumeric).to_string()).collect();
    let secret_key_base = format!("SECRET_KEY_BASE={}", random_string);

    let env = vec![
        "RPC_API_HOST=http://host.docker.internal:9944",
        "DB_TYPE=sqlite",
        "DISABLE_MAINNET_SYNC=false",
        "DISABLE_TESTNET_SYNC=true",
        "TESTNET_RPC_API_HOST=http://host.docker.internal:9944",
        "DATABASE_PATH=/use/exp.db",
        "PHX_HOST=localhost",
        &secret_key_base,
    ];

    let mut port_bindings = HashMap::new();
    port_bindings.insert(
        "4000/tcp".to_string(),
        Some(vec![PortBinding {
            host_ip: Some("0.0.0.0".to_string()),
            host_port: Some("4000".to_string()),
        }]),
    );

    let host_config = HostConfig { port_bindings: Some(port_bindings), ..Default::default() };

    run_docker_image(
        "ghcr.io/lambdaclass/stark_compass_explorer:v0.2.34.2",
        "madara-explorer",
        Some(env),
        Some(host_config),
    );
}
