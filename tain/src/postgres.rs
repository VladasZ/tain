use anyhow::{anyhow, Result};

use crate::{ContainerConfig, Docker, Mount, Port};

pub struct Postgres {}

impl Postgres {
    pub fn start(name: &str) -> Result<()> {
        let home = dirs::home_dir().ok_or(anyhow!("no HOME"))?;
        let home = home.to_str().unwrap();

        let source = format!("{home}/spesogon_pg");
        let pg_data = "/spesogon_pg";

        Docker::start(
            ContainerConfig::builder()
                .name(name)
                .image("postgres:16.2-alpine")
                .port(Port {
                    host:      5432,
                    container: 5432,
                })
                .mount(Mount {
                    host:      source,
                    container: pg_data.into(),
                })
                .env([
                    ("POSTGRES_PASSWORD".to_string(), "1111".to_string()),
                    ("POSTGRES_DB".to_string(), "spesogon".to_string()),
                    ("PGDATA".to_string(), pg_data.to_string()),
                ])
                .build(),
        )?;

        Ok(())
    }
}
