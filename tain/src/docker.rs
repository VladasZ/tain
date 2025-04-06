use std::{fs::create_dir_all, process::Command};

use anyhow::{Result, bail};
use log::info;
use serde_json::from_str;

use crate::{ContainerConfig, container::DockerContainer};

pub struct Docker {}

impl Docker {
    pub fn check_running() -> Result<()> {
        if !Command::new("docker").arg("info").output()?.status.success() {
            bail!("Docker is not running")
        }
        info!("Docker: OK");
        Ok(())
    }

    pub fn all_containers() -> Result<Vec<DockerContainer>> {
        let output = Command::new("docker")
            .arg("ps")
            .arg("--format")
            .arg("json")
            .arg("--all")
            .output()?;
        if !output.status.success() {
            bail!(String::from_utf8(output.stderr).unwrap());
        }

        let json = String::from_utf8(output.stdout)?;
        let json: String = json.split('\n').filter(|a| !a.is_empty()).collect::<Vec<_>>().join(",\n");
        let json = format!("[\n{json}\n]");

        let containers: Vec<DockerContainer> = from_str(&json)?;

        Ok(containers)
    }

    pub fn running(name: &str) -> Result<bool> {
        let Some(container) = Self::get(name)? else {
            return Ok(false);
        };

        Ok(container.running())
    }

    pub fn get(name: &str) -> Result<Option<DockerContainer>> {
        Ok(Self::all_containers()?.into_iter().find(|c| c.names == name))
    }

    pub fn start_existing(name: &str) -> Result<()> {
        let output = Command::new("docker").arg("start").arg(name).output()?;

        assert!(output.status.success(), "{output:?}");

        Ok(())
    }

    pub fn start(config: ContainerConfig) -> Result<()> {
        if let Some(container) = Docker::get(&config.name)? {
            if container.running() {
                return Ok(());
            }

            return container.start_or_unpause();
        }

        let mut command = Command::new("docker");

        command
            .arg("run")
            .arg("--name")
            .arg(config.name)
            .arg("--cap-add=SYS_PTRACE")
            .arg("--security-opt")
            .arg("seccomp=unconfined")
            .arg("-p")
            .arg(format!("{}:{}", config.port.host, config.port.container));

        if let Some(mount) = config.mount {
            let host = &mount.host;

            if !host.exists() {
                create_dir_all(host)?;
            }

            command.arg("--mount").arg(format!(
                "type=bind,source={},target={}",
                mount.host.display(),
                mount.container.display()
            ));
        }

        for (key, value) in config.env {
            command.arg("-e").arg(format!("{key}={value}"));
        }

        command.arg("--detach").arg(config.image);

        let output = command.output()?;

        if !output.status.success() {
            bail!(String::from_utf8(output.stderr).unwrap());
        }

        Ok(())
    }
}
