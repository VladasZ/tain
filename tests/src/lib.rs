#![cfg(test)]

use anyhow::{anyhow, Result};
use tain::{Docker, Mount, Postgres, PostgresConfig};

#[test]
fn test() -> Result<()> {
    Docker::check_running()?;

    assert!(!Docker::running("tain_pg_test")?);

    let home = dirs::home_dir().ok_or(anyhow!("no HOME"))?;
    let home = home.to_str().unwrap();

    let source = format!("{home}/spesogon_pg");
    let pg_data = "/spesogon_pg";

    Postgres::start(
        PostgresConfig::builder()
            .container_name("tain_pg_test")
            .data(Mount {
                host:      source,
                container: pg_data.to_string(),
            })
            .build(),
    )?;

    assert!(Docker::running("tain_pg_test")?);

    Docker::rm("tain_pg_test")?;

    assert!(!Docker::running("tain_pg_test")?);

    Ok(())
}
