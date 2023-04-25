use std::path::PathBuf;

pub struct GitRepository {
    pub(crate) path: PathBuf,
}

impl GitRepository {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}
