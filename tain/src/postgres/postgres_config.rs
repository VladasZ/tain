use typed_builder::TypedBuilder;

use crate::Mount;

#[derive(TypedBuilder)]
pub struct PostgresConfig {
    #[builder(setter(into))]
    pub container_name: String,
    #[builder(default, setter(strip_option))]
    pub db:             Option<String>,
    #[builder(default, setter(strip_option))]
    pub password:       Option<String>,
    #[builder(default, setter(strip_option))]
    pub data:           Option<Mount>,
}
