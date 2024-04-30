mod docker;
mod postgres;
mod static_get;

pub use static_get::{static_drop, static_get};
pub use testcontainers::{clients::Cli as Docker, Container};

pub use crate::postgres::{Postgres, PostgresArc};
