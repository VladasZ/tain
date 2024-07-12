mod postgres;
mod static_arc;
mod static_get;

pub use static_get::Static;
pub use testcontainers::{core::ImageExt, runners::AsyncRunner, ContainerRequest};

pub use crate::postgres::Postgres;

pub type Container<T> = testcontainers::ContainerAsync<T>;
