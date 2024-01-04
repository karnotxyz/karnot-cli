use crate::utils::docker::run_docker_image;

pub fn explorer() {
    let env_vars = vec!["RPC_API_HOST=\"http://127.0.0.1:9944\""];
    run_docker_image("ghcr.io/lambdaclass/stark_compass_explorer:v0.2.34.2", "madara-explorer", env_vars);
}
