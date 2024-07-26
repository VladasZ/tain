use std::collections::HashMap;

use typed_builder::TypedBuilder;

pub struct Mount {
    pub host:      String,
    pub container: String,
}

pub struct Port {
    pub host:      u16,
    pub container: u16,
}

#[derive(TypedBuilder)]
pub struct ContainerConfig {
    #[builder(setter(into))]
    pub name:  String,
    #[builder(setter(into))]
    pub image: String,
    pub port:  Port,
    #[builder(default, setter(strip_option))]
    pub mount: Option<Mount>,
    #[builder(default, setter(into))]
    pub env:   HashMap<String, String>,
}
