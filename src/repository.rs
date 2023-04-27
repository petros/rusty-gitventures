use git2::Repository;
use git2::{BranchType, Error};
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

    pub fn list_local_branches(&self) -> Result<(), Error> {
        let branches = self.repo.branches(None)?;

        for branch in branches {
            let (branch, branch_type) = branch?;
            if branch_type == BranchType::Local {
                if let Some(name) = branch.name()? {
                    println!("  {}", name);
                }
            }
        }

        Ok(())
    }

    pub fn show_current_branch(&self) -> Result<(), Error> {
        let head = self.repo.head()?;
        if let Some(branch) = head.shorthand() {
            println!("  {}", branch);
        } else {
            eprintln!("The current branch is not a valid UTF-8 string.");
        }
        Ok(())
    }

    pub fn show_status(&self) -> Result<(), Error> {
        let statuses = self.repo.statuses(None)?;
        for entry in statuses.iter() {
            let status = entry.status();
            let status_description = self.status_description(status);
            let path = entry.path().unwrap();
            println!("  {} - {}", path, status_description);
        }
        Ok(())
    }

    fn status_description(&self, status: git2::Status) -> &'static str {
        match status {
            // No changes
            git2::Status::CURRENT => "current",
            // A new file has been added to the index.
            git2::Status::INDEX_NEW => "new in index",
            // A file in the index has been modified.
            git2::Status::INDEX_MODIFIED => "modified in index",
            // A file has been deleted from the index.
            git2::Status::INDEX_DELETED => "deleted from index",
            // A file in the index has been renamed.
            git2::Status::INDEX_RENAMED => "renamed in index",
            // A file in the index had had its type changed, e.g., from a regular file to a symbolic link.
            git2::Status::INDEX_TYPECHANGE => "typechange in index",
            // A new file exists in the working directory but is not yet tracked in the index.
            git2::Status::WT_NEW => "new in working tree",
            // A tracked file in the working directory has been modified but not yet staged.
            git2::Status::WT_MODIFIED => "modified in working tree",
            // A tracked file has been deleted from the working directory but not yet removed from the index.
            git2::Status::WT_DELETED => "deleted from working tree",
            // A tracked file in the working directory has had its type changed but not yet staged,
            // e.g., from a regular file to a symbolic link.
            git2::Status::WT_TYPECHANGE => "typechange in working tree",
            // A tracked file in the working directory has been renamed but not yet staged.
            git2::Status::WT_RENAMED => "renamed in working tree",
            // The file is ignored, typically based on the rules in the `.gitignore` file.
            git2::Status::IGNORED => "ignored",
            // The file has a conflict between the index and the working directory, usually due to a merge conflict.
            git2::Status::CONFLICTED => "conflicted",
            _ => "unknown",
        }
    }
}
