use bollard::container::{Config, CreateContainerOptions};
use bollard::image::CreateImageOptions;
use bollard::models::HostConfig;
use bollard::Docker;
use futures_util::TryStreamExt;

pub fn run_docker_image(image: &str, container_name: &str, env: Option<Vec<&str>>, host_config: Option<HostConfig>) {
    is_docker_installed();
    match pull_and_start_docker_image(image, container_name, env, host_config) {
        Ok(..) => {
            log::info!("Successfully ran {}", container_name);
        }
        Err(err) => {
            log::error!("Error: {} running image: {}", err, image);
        }
    };
}

#[tokio::main]
async fn is_docker_installed() -> bool {
    let docker = Docker::connect_with_local_defaults().unwrap();
    return match docker.version().await {
        Ok(_) => {
            log::info!("Docker running!");
            true
        }
        Err(_) => {
            panic!("Please check docker installation, panicking");
        }
    };
}

#[tokio::main]
async fn pull_and_start_docker_image(
    image: &str,
    container_name: &str,
    env: Option<Vec<&str>>,
    host_config: Option<HostConfig>,
) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let docker = Docker::connect_with_local_defaults().unwrap();

    docker
        .create_image(Some(CreateImageOptions { from_image: image, ..Default::default() }), None, None)
        .try_collect::<Vec<_>>()
        .await?;


    let config = Config { image: Some(image), tty: Some(true), env, host_config, ..Default::default() };

    let container_option = Some(CreateContainerOptions { name: container_name, ..Default::default() });

    let id = docker.create_container::<&str, &str>(container_option, config).await?.id;

    docker.start_container::<String>(&id, None).await?;

    Ok(())
}
