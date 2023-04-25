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
                "Repository opened successfully with a path of {}!",
                git_repo.path().to_str().unwrap()
            );
            println!("Path: {:?}", git_repo.repository_path());

            println!("");
            println!("List all remotes...");
            git_repo.list_remotes();
        }
        Err(e) => {
            eprintln!("Error opening repository: {:?}", e);
        }
    }
}
