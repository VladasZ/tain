mod docker;
mod postgres;
mod static_arc;
mod static_get;

pub use static_get::Static;
pub use testcontainers::{clients::Cli as Docker, Container};

pub use crate::postgres::{Postgres, PostgresArc};
