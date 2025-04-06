#![cfg(test)]

use std::{path::PathBuf, str::FromStr};

use anyhow::{Result, anyhow};
use serial_test::serial;
use tain::{Docker, Mount, Postgres, PostgresConfig};

#[test]
#[serial]
fn test_builder() -> Result<()> {
    Docker::check_running()?;

    assert!(!Docker::running("tain_pg_test")?);

    let home = dirs::home_dir().ok_or(anyhow!("no HOME"))?;

    let host_pg_path = home.join("spesogon_pg");

    let config = PostgresConfig::builder()
        .container_name("tain_pg_test")
        .data(Mount {
            host:      host_pg_path.clone(),
            container: PathBuf::from_str("/spesogon_pg")?,
        })
        .build();

    Postgres::start(config.clone())?;

    let container = Docker::get("tain_pg_test")?.unwrap();

    assert!(Docker::running("tain_pg_test")?);

    container.rm()?;

    assert!(!Docker::running("tain_pg_test")?);

    assert!(host_pg_path.exists());

    Postgres::start(config.clone())?;

    assert!(Docker::running("tain_pg_test")?);
    assert!(host_pg_path.exists());

    Postgres::wipe_container(config)?;

    assert!(!Docker::running("tain_pg_test")?);
    assert!(!host_pg_path.exists());

    Ok(())
}

#[test]
#[serial]
fn test_env() -> Result<()> {
    Docker::check_running()?;

    assert!(!Docker::running("tain_test_env_pg")?);

    Postgres::start_env()?;

    assert!(Docker::running("tain_test_env_pg")?);

    let container = Docker::get("tain_test_env_pg")?.unwrap();

    container.rm()?;

    assert!(!Docker::running("tain_test_env_pg")?);

    Ok(())
}

#[test]
#[serial]
fn start_stopped_if_exists() -> Result<()> {
    Docker::check_running()?;

    if let Some(container) = Docker::get("tain_test_env_pg")? {
        container.rm()?
    }

    let config = PostgresConfig::from_env()?;

    Postgres::start(&config)?;

    let container = Docker::get(&config.container_name)?.unwrap();

    container.pause()?;

    assert!(Docker::get(&config.container_name)?.unwrap().paused());

    Postgres::start(&config)?;

    assert!(Docker::get(&config.container_name)?.unwrap().running());

    container.stop()?;

    assert!(Docker::get(&config.container_name)?.unwrap().stopped());

    Postgres::start(&config)?;

    assert!(Docker::get(&config.container_name)?.unwrap().running());

    container.rm()?;

    assert_eq!(Docker::get(&config.container_name)?, None);

    Ok(())
}

#[test]
fn connection_string() -> Result<()> {
    assert_eq!(
        Postgres::connection_string()?,
        "postgresql://postgres:tain_test_env_pg@localhost:54320/tain_test_env_pg"
    );
    Ok(())
}
