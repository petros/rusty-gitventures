use repository::Repository;
use std::path::PathBuf;

mod repository;

fn main() {
    let path = PathBuf::from("~/Projects/petros/rusty-gitventures");
    let repository = Repository::new(path);
    println!("Repository path: {}", repository.path.to_string_lossy());
}
