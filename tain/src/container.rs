use std::process::Command;

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DockerContainer {
    pub command:       String,
    pub created_at:    String,
    #[serde(rename = "ID")]
    pub id:            String,
    pub image:         String,
    pub labels:        String,
    pub local_volumes: String,
    pub mounts:        String,
    pub names:         String,
    pub networks:      String,
    pub ports:         String,
    pub running_for:   String,
    pub size:          String,
    pub state:         String,
    pub status:        String,
}

impl DockerContainer {
    pub fn running(&self) -> bool {
        self.state == "running"
    }

    pub fn paused(&self) -> bool {
        self.state == "paused"
    }

    pub fn stopped(&self) -> bool {
        self.state == "exited"
    }

    pub fn stop(&self) -> Result<()> {
        self.docker_command("stop")
    }

    pub fn start(&self) -> Result<()> {
        self.docker_command("start")
    }

    pub fn pause(&self) -> Result<()> {
        self.docker_command("pause")
    }

    pub fn unpause(&self) -> Result<()> {
        self.docker_command("unpause")
    }

    pub fn start_or_unpause(&self) -> Result<()> {
        if self.running() {
            return Ok(());
        }

        if self.paused() {
            return self.unpause();
        }

        if self.stopped() {
            return self.start();
        }

        panic!("Invalid container state")
    }

    pub fn rm(self) -> Result<()> {
        self.docker_command("stop")?;
        self.docker_command("rm")
    }

    fn docker_command(&self, command: &str) -> Result<()> {
        let output = Command::new("docker").arg(command).arg(&self.names).output()?;

        assert!(output.status.success(), "{output:?}");

        Ok(())
    }
}
