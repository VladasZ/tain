use std::collections::HashMap;

use anyhow::{anyhow, Result};
use dotenvy::{dotenv, vars};
use typed_builder::TypedBuilder;

use crate::Mount;

#[derive(TypedBuilder)]
pub struct PostgresConfig {
    #[builder(setter(into))]
    pub container_name: String,

    #[builder(default, setter(strip_option), setter(into))]
    pub db: Option<String>,

    #[builder(default, setter(strip_option), setter(into))]
    pub password: Option<String>,

    #[builder(default, setter(strip_option))]
    pub data: Option<Mount>,
}

impl PostgresConfig {
    pub fn from_env() -> Result<Self> {
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

        let new = Self::builder()
            .container_name(container_name)
            .db(db)
            .password(password)
            .data(Mount {
                host:      data_host.clone(),
                container: data_container.clone(),
            })
            .build();

        Ok(new)
    }
}
