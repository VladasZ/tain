mod postgres;
mod static_arc;
mod static_get;

pub use static_get::Static;
pub use testcontainers::runners::AsyncRunner;

pub use crate::postgres::Postgres;
