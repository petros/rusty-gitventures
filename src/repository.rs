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
            let remote = remote.unwrap();
            let remote_url: String = match self.repo.find_remote(remote) {
                Ok(remote) => remote.url().unwrap().to_string(),
                Err(e) => {
                    eprintln!("  Error finding remote information: {:?}", e);
                    "".to_string()
                }
            };
            println!("  {} / {}", remote, remote_url);
        }
    }

    pub fn list_local_branches(&self) {
        let branches = self.repo.branches(None).unwrap();
        for branch in branches {
            let (branch, branch_type) = branch.unwrap();
            if branch_type == git2::BranchType::Local {
                println!("  {}", branch.name().unwrap().unwrap());
            }
        }
    }
}
