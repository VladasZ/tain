#![cfg(test)]

use anyhow::Result;
use tain::{Docker, Postgres};

#[test]
fn test() -> Result<()> {
    Docker::check_running()?;

    assert!(!Docker::running("tain_pg_test")?);

    Postgres::start("tain_pg_test")?;

    assert!(Docker::running("tain_pg_test")?);

    Docker::rm("tain_pg_test")?;

    assert!(!Docker::running("tain_pg_test")?);

    Ok(())
}
