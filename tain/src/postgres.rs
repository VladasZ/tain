use std::collections::HashMap;

use testcontainers::{core::WaitFor, Image};

const NAME: &str = "postgres";
const TAG: &str = "16-alpine";

/// Module to work with [`Postgres`] inside of tests.
///
/// Starts an instance of Postgres.
/// This module is based on the official [`Postgres docker image`].
///
/// Default db name, user and password is `postgres`.
///
/// # Example
/// ```
/// use testcontainers::clients;
/// use testcontainers_modules::postgres;
///
/// let docker = clients::Cli::default();
/// let postgres_instance = docker.run(postgres::Postgres::default());
///
/// let connection_string = format!(
///     "postgres://postgres:postgres@127.0.0.1:{}/postgres",
///     postgres_instance.get_host_port_ipv4(5432)
/// );
/// ```
///
/// [`Postgres`]: https://www.postgresql.org/
/// [`Postgres docker image`]: https://hub.docker.com/_/postgres
#[derive(Debug)]
pub struct Postgres {
    env_vars: HashMap<String, String>,
}

impl Postgres {
    //
    // /// Sets the db name for the Postgres instance.
    // pub fn with_db_name(mut self, db_name: &str) -> Self {
    //     self.env_vars.insert("POSTGRES_DB".to_owned(), db_name.to_owned());
    //     self
    // }
    //
    // /// Sets the user for the Postgres instance.
    // pub fn with_user(mut self, user: &str) -> Self {
    //     self.env_vars.insert("POSTGRES_USER".to_owned(), user.to_owned());
    //     self
    // }
    //
    // /// Sets the password for the Postgres instance.
    // pub fn with_password(mut self, password: &str) -> Self {
    //     self.env_vars.insert("POSTGRES_PASSWORD".to_owned(), password.to_owned());
    //     self
    // }
}

impl Default for Postgres {
    fn default() -> Self {
        let mut env_vars = HashMap::new();
        env_vars.insert("POSTGRES_DB".to_owned(), "postgres".to_owned());
        env_vars.insert("POSTGRES_USER".to_owned(), "postgres".to_owned());
        env_vars.insert("POSTGRES_PASSWORD".to_owned(), "postgres".to_owned());

        Self { env_vars }
    }
}

impl Image for Postgres {
    type Args = ();

    fn name(&self) -> String {
        NAME.to_owned()
    }

    fn tag(&self) -> String {
        TAG.to_owned()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        )]
    }

    fn env_vars(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        Box::new(self.env_vars.iter())
    }
}
