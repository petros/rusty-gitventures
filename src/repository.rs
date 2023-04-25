use std::path::PathBuf;

pub struct Repository {
    pub(crate) path: PathBuf,
}

impl Repository {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}
