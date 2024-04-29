mod docker;
mod postgres;
mod r#static;

pub use testcontainers::{clients::Cli as Docker, Container};

pub use crate::postgres::{Postgres, PostgresArc};
