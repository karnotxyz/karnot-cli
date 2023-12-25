use env_logger;

pub fn setup_logger() {
    let mut log_builder = env_logger::Builder::new();
    let mut env = env_logger::Env::new();
    env = env.filter("KARNOT_LOG_LEVEL").default_filter_or("INFO");
    log_builder.parse_env(env);
    log_builder.init();
}

pub mod cli;

pub mod app;
