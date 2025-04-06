use std::{borrow::Borrow, collections::HashMap, fs::remove_dir_all};

use anyhow::{Result, anyhow};
use dotenvy::vars;

use crate::{ContainerConfig, Docker, Port, PostgresConfig};

pub struct Postgres {}

impl Postgres {
    pub fn connection_string() -> Result<String> {
        let config = PostgresConfig::from_env()?;

        let vars: HashMap<String, String> = vars().collect();

        let password = config.password.unwrap_or("postgres".to_string());
        let host = vars.get("POSTGRES_HOST").ok_or(anyhow!("No POSTGRES_HOST in .env"))?;
        let db = config.db.unwrap_or("postgres".to_string());

        Ok(format!("postgresql://postgres:{password}@{host}/{db}"))
    }

    pub fn start_env() -> Result<()> {
        Self::start(PostgresConfig::from_env()?)?;

        Ok(())
    }

    pub fn start(config: impl Borrow<PostgresConfig>) -> Result<()> {
        let config = config.borrow().clone();
        let container = ContainerConfig::builder()
            .name(config.container_name)
            .image("postgres:16.2-alpine")
            .port(Port {
                host:      config.port,
                container: 5432,
            });

        let mut env: HashMap<_, _> = [
            (
                "POSTGRES_PASSWORD".to_string(),
                config.password.unwrap_or("postgres".to_string()),
            ),
            (
                "POSTGRES_DB".to_string(),
                config.db.unwrap_or("postgres".to_string()),
            ),
        ]
        .into();

        let mut config = if let Some(data) = config.data {
            env.insert("PGDATA".to_string(), data.container.to_str().unwrap().to_string());
            container.mount(data).build()
        } else {
            container.build()
        };

        config.env = env;

        Docker::start(config)?;

        Ok(())
    }

    pub fn wipe_container(config: PostgresConfig) -> Result<()> {
        if let Some(container) = Docker::get(&config.container_name)? {
            container.rm()?;
        }

        if let Some(mount) = config.data {
            if mount.host.exists() {
                remove_dir_all(mount.host)?;
            }
        }

        Ok(())
    }

    pub fn wipe_container_env() -> Result<()> {
        Self::wipe_container(PostgresConfig::from_env()?)
    }
}
