mod postgres;

use anyhow::Result;
use testcontainers::clients::Cli;

use crate::postgres::Postgres;

pub async fn request_containter() -> Result<()> {
    let docker = Cli::default();
    let postgres = docker.run(Postgres::default());

    dbg!(&postgres);

    Ok(())
}
