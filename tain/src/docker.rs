use std::process::Command;

use anyhow::{bail, Result};
use log::info;
use serde_json::from_str;

use crate::container::DockerContainer;

pub struct Docker {}

impl Docker {
    pub fn check_running() -> Result<()> {
        if !Command::new("docker").arg("info").output()?.status.success() {
            bail!("Docker is not running")
        }
        info!("Docker: OK");
        Ok(())
    }

    pub fn running_containers() -> Result<Vec<DockerContainer>> {
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
        let containers = Self::running_containers()?;
        Ok(containers.into_iter().any(|c| c.names == name))
    }

    pub fn rm(name: &str) -> Result<()> {
        assert!(Command::new("docker").arg("stop").arg(name).output()?.status.success());
        assert!(Command::new("docker").arg("rm").arg(name).output()?.status.success());

        Ok(())
    }
}
