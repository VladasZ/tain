use std::collections::HashMap;

pub struct Mount {
    pub host:      String,
    pub container: String,
}

pub struct Port {
    pub host:      u16,
    pub container: u16,
}

pub struct ContainerConfig {
    pub name:  String,
    pub image: String,
    pub port:  Port,
    pub mount: Option<Mount>,
    pub env:   HashMap<String, String>,
}
