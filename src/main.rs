use std::{env, fs, path::PathBuf};

fn main() {
    let path = env::current_dir().unwrap();
    let items = get_dir_items(&path);
    println!("{:?}", items)
}

fn get_dir_items(path: &PathBuf) -> Vec<PathBuf> {
    fs::read_dir(path)
        .unwrap()
        .map(|item| item.unwrap())
        .map(|item| item.path())
        .collect()
}
