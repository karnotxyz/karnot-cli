use bollard::container::{Config, CreateContainerOptions, ListContainersOptions};
use bollard::image::CreateImageOptions;
use bollard::models::HostConfig;
use bollard::Docker;
use futures_util::TryStreamExt;

// TODO: fix unwraps and handle errors

pub async fn run_docker_image(
    image: &str,
    container_name: &str,
    env: Option<Vec<&str>>,
    host_config: Option<HostConfig>,
    start_cmd: Option<Vec<&str>>,
) {
    is_docker_installed().await;
    log::info!("🐳 Running docker image: {}", image);
    match pull_and_start_docker_image(image, container_name, env, host_config, start_cmd).await {
        Ok(..) => {
            log::debug!("Successfully ran {}", container_name);
        }
        Err(err) => {
            log::error!("Error: {} running image: {}", err, image);
        }
    };
}

async fn is_docker_installed() {
    let docker = Docker::connect_with_local_defaults().unwrap();
    match docker.version().await {
        Ok(_) => {
            log::debug!("✅ Docker is installed!");
            true
        }
        Err(_) => {
            panic!("Please check docker installation, panicking");
        }
    };
}

pub async fn container_exists(container_name: &str) -> bool {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let list_container_options = ListContainersOptions { all: true, ..Default::default() };
    match docker.list_containers::<String>(Some(list_container_options)).await {
        Ok(containers) => {
            for container in containers {
                if let Some(names) = container.names {
                    if names.contains(&format!("/{}", &container_name.to_string())) {
                        log::debug!("✅ Container {} exists!", container_name);
                        return true;
                    }
                }
            }
            log::debug!("❌ Container {} does not exist!", container_name);
            false
        }
        Err(_) => {
            panic!("Failed to fetch containers, panicking");
        }
    }
}

pub async fn is_container_running(container_name: &str) -> bool {
    let docker = Docker::connect_with_local_defaults().unwrap();

    if let Some(state) = docker.inspect_container(container_name, None).await.unwrap_or_default().state {
            return state.running.unwrap_or(false);
    }

    false
}

pub async fn kill_container(container_name: &str) -> eyre::Result<()> {
    let docker = Docker::connect_with_local_defaults().unwrap();
    // TODO: handle the error
    let _ = docker.kill_container::<String>(container_name, None).await;
    docker.remove_container(container_name, None).await?;
    Ok(())
}

pub async fn pull_and_start_docker_image(
    image: &str,
    container_name: &str,
    env: Option<Vec<&str>>,
    host_config: Option<HostConfig>,
    start_cmd: Option<Vec<&str>>,
) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let docker = Docker::connect_with_local_defaults().unwrap();

    docker
        .create_image(Some(CreateImageOptions { from_image: image, ..Default::default() }), None, None)
        .try_collect::<Vec<_>>()
        .await?;

    let config = Config { image: Some(image), cmd: start_cmd, tty: Some(true), env, host_config, ..Default::default() };

    let container_option = Some(CreateContainerOptions { name: container_name, ..Default::default() });

    let id = docker.create_container::<&str, &str>(container_option, config).await?.id;

    docker.start_container::<String>(&id, None).await?;

    Ok(())
}
