use l::Files;
use std::env;

fn main() {
    let path = env::current_dir().expect("Cannot read current directory");
    let items = Files::from_path(path);
    items.render();
}
