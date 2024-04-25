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
    port: u16,
}

impl Postgres {
    pub fn db(mut self, db_name: impl ToString) -> Self {
        self.env_vars.insert("POSTGRES_DB".to_owned(), db_name.to_string());
        self
    }

    pub fn user(mut self, user: impl ToString) -> Self {
        self.env_vars.insert("POSTGRES_USER".to_owned(), user.to_string());
        self
    }

    pub fn password(mut self, password: impl ToString) -> Self {
        self.env_vars.insert("POSTGRES_PASSWORD".to_owned(), password.to_string());
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn data(mut self, password: impl ToString) -> Self {
        self.env_vars.insert("PGDATA".to_owned(), password.to_string());
        self
    }
}

impl Default for Postgres {
    fn default() -> Self {
        let mut env_vars = HashMap::new();
        env_vars.insert("POSTGRES_DB".to_owned(), "postgres".to_owned());
        env_vars.insert("POSTGRES_USER".to_owned(), "postgres".to_owned());
        env_vars.insert("POSTGRES_PASSWORD".to_owned(), "postgres".to_owned());

        Self { env_vars, port: 5432 }
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

    fn expose_ports(&self) -> Vec<u16> {
        vec![self.port]
    }
}
