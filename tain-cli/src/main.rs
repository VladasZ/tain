use anyhow::Result;
use structopt::StructOpt;
use tain::{Docker, PostgresConfig};

#[derive(StructOpt, Debug)]
#[structopt(name = "cli")]
struct Cli {
    #[structopt(subcommand)]
    command: Commands,
}

#[derive(StructOpt, Debug)]
enum Commands {
    Postgres(Postgres),
}

#[derive(StructOpt, Debug)]
struct Postgres {
    #[structopt(subcommand)]
    subcommand: Option<PostgresCommand>,
}

#[derive(StructOpt, Debug)]
enum PostgresCommand {
    Status,
    Rm,
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    match args.command {
        Commands::Postgres(pg) => match pg.subcommand {
            Some(PostgresCommand::Status) => {
                println!("Postgres status:");
                let Some(postgres) = Docker::get(&PostgresConfig::from_env()?.container_name)? else {
                    println!("Doesn't exist");
                    return Ok(());
                };
                println!("{}", postgres.state);
                println!("{}", postgres.status);
            }
            Some(PostgresCommand::Rm) => {
                println!("Removing postgres container and data directory");
                let Some(postgres) = Docker::get(&PostgresConfig::from_env()?.container_name)? else {
                    println!("Doesn't exist");
                    return Ok(());
                };

                postgres.rm()?;

                println!("Removed")
            }
            None => {
                println!("Starting postgres container");
                tain::Postgres::start_env()?;
            }
        },
    }

    Ok(())
}
