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

    assert!(Docker::running("tain_pg_test")?);

    Docker::rm("tain_pg_test")?;

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

    Docker::rm("tain_test_env_pg")?;

    assert!(!Docker::running("tain_test_env_pg")?);

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
