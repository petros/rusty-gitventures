use git2::Repository;
use std::path::PathBuf;

pub struct GitRepository {
    path: PathBuf,
    repo: Repository,
}

impl GitRepository {
    pub fn new(path: PathBuf) -> Result<Self, git2::Error> {
        let repo = Repository::open(&path)?;
        Ok(Self { path, repo })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn repository_path(&self) -> String {
        self.repo.path().to_str().unwrap().to_string()
    }

    pub fn list_remotes(&self) {
        let remotes = self.repo.remotes().unwrap();
        for remote in remotes.iter() {
            println!("{}", remote.unwrap());
            match self.repo.find_remote(remote.unwrap()) {
                Ok(remote) => println!("  {}", remote.url().unwrap()),
                Err(e) => eprintln!("  Error finding remote information: {:?}", e),
            }
        }
    }
}
