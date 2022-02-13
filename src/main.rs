use std::{env, fs, path::PathBuf};

const BLUE_BOLD: &str = "\x1b[94;1m";

fn get_dir_items(path: &PathBuf) -> Vec<PathBuf> {
    fs::read_dir(path)
        .unwrap()
        .map(|item| item.unwrap())
        .map(|item| item.path())
        .collect()
}

fn render_items(items: Vec<PathBuf>) {
    for item in items {
        if item.is_dir() {
            println!(
                "• {}{}\x1b[0m",
                &BLUE_BOLD,
                item.file_name().unwrap().to_str().unwrap()
            )
        } else {
            println!("• { }", item.file_name().unwrap().to_str().unwrap())
        }
    }
}

fn main() {
    let path = env::current_dir().unwrap();
    let items = get_dir_items(&path);
    render_items(items);
}
