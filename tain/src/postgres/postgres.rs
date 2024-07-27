use std::collections::HashMap;

use anyhow::Result;

use crate::{ContainerConfig, Docker, Port, PostgresConfig};

pub struct Postgres {}

impl Postgres {
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
