use repository::GitRepository;
use std::path::PathBuf;

mod repository;

fn main() {
    let path = PathBuf::from("/Users/petros/Projects/petros/rusty-gitventures");
    println!("Playing with git2");
    println!("-----------------");
    println!("");

    println!("Creating repository...");
    match GitRepository::new(path) {
        Ok(git_repo) => {
            println!(
                "  Repository opened successfully with a path of '{}'",
                git_repo.path().to_str().unwrap()
            );
            println!("  Path: '{}'", git_repo.repository_path());

            println!("");
            println!("List all remotes...");
            git_repo.list_remotes();

            println!("");
            println!("List all branches...");
            if let Err(e) = git_repo.list_local_branches() {
                eprintln!("Error listing local branches: {:?}", e);
            }

            println!("");
            println!("Show current branch...");
            if let Err(e) = git_repo.show_current_branch() {
                eprintln!("Error showing current branch: {:?}", e);
            }
        }
        Err(e) => {
            eprintln!("Error opening repository: {:?}", e);
        }
    }
}
