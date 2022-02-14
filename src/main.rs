use l::{get_dir_items, render_items};
use std::env;

fn main() {
    let path = env::current_dir().unwrap();
    let items = get_dir_items(&path);
    render_items(items);
}
