use std::sync::OnceLock;

use crate::Docker;

static DOCKER: OnceLock<Docker> = OnceLock::new();

pub(crate) fn docker() -> &'static Docker {
    DOCKER.get_or_init(|| Docker::default())
}
