use repository::GitRepository;
use std::path::PathBuf;

mod repository;

fn main() {
    let path = PathBuf::from("~/Projects/petros/rusty-gitventures");
    let repository = GitRepository::new(path);
    println!("Repository path: {}", repository.path.to_string_lossy());
}
