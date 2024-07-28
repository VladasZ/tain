use std::collections::HashMap;

use anyhow::{anyhow, Result};
use dotenvy::{dotenv, vars};

use crate::{ContainerConfig, Docker, Mount, Port, PostgresConfig};

pub struct Postgres {}

impl Postgres {
    pub fn start_env() -> Result<()> {
        dotenv()?;

        let vars: HashMap<String, String> = vars().collect();

        let db = vars.get("POSTGRES_DB").ok_or(anyhow!("No POSTGRES_DB in .env"))?;
        let password = vars.get("POSTGRES_PASSWORD").ok_or(anyhow!("No POSTGRES_PASSWORD in .env"))?;
        let data_host = vars.get("POSTGRES_DATA_HOST").ok_or(anyhow!("No POSTGRES_DATA_HOST in .env"))?;
        let data_container = vars
            .get("POSTGRES_DATA_CONTAINER")
            .ok_or(anyhow!("No POSTGRES_DATA_CONTAINER in .env"))?;
        let container_name = vars
            .get("POSTGRES_CONTAINER_NAME")
            .ok_or(anyhow!("No POSTGRES_CONTAINER_NAME in .env"))?;

        Self::start(
            PostgresConfig::builder()
                .container_name(container_name)
                .db(db)
                .password(password)
                .data(Mount {
                    host:      data_host.clone(),
                    container: data_container.clone(),
                })
                .build(),
        )?;

        Ok(())
    }

    pub fn start(config: PostgresConfig) -> Result<()> {
        let container = ContainerConfig::builder()
            .name(config.container_name)
            .image("postgres:16.2-alpine")
            .port(Port::postgres());

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

        let config = if let Some(data) = config.data {
            env.insert("PGDATA".to_string(), data.container.clone());
            container.mount(data).build()
        } else {
            container.build()
        };

        Docker::start(config)?;

        Ok(())
    }
}
