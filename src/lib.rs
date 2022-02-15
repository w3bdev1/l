use std::{os::unix::prelude::MetadataExt, path::PathBuf};

// Colors
const RESET_COLOR: &str = "\x1b[0m";
const DIR_COLOR: &str = "\x1b[94;1m";
const SIZE_COLOR: &str = "\x1b[95m";

#[derive(Debug)]
pub struct Files {
    files: Vec<File>,
}

impl Files {
    fn new(files: Vec<File>) -> Files {
        Files { files }
    }

    pub fn from_path(path: PathBuf) -> Files {
        let mut files: Vec<File> = Vec::new();
        for item in path.read_dir().expect("Could not read the directory") {
            if let Ok(item) = item {
                let item_path = item.path();
                let item_metadata = item.metadata().unwrap();
                let name = String::from(item_path.file_name().unwrap().to_str().unwrap());
                let is_dir = item_path.is_dir();
                let size = human_readable_size(item_metadata.size());

                files.push(File { name, is_dir, size })
            }
        }

        Files::new(files)
    }

    pub fn render(self) {
        for f in self.files {
            println!("{}", print_file(&f, f.is_dir))
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    is_dir: bool,
    size: String,
}

fn print_file(f: &File, is_dir: bool) -> String {
    if is_dir {
        format!(
            "• {size_color}{size} {dir_color}{dir_name}{reset_color}",
            size_color = SIZE_COLOR,
            size = f.size,
            dir_color = DIR_COLOR,
            dir_name = f.name,
            reset_color = RESET_COLOR
        )
    } else {
        format!(
            "• {size_color}{size}{reset_color} {file_name}",
            size_color = SIZE_COLOR,
            size = f.size,
            reset_color = RESET_COLOR,
            file_name = f.name
        )
    }
}

fn human_readable_size(size: u64) -> String {
    let mut size = size;
    let units = ['B', 'K', 'M', 'G', 'T', 'P', 'E'];
    let mut unit_index = 0;

    while size > 1000 {
        size = size / 1000;
        unit_index += 1;
    }

    format!("{:4}{}", size, units[unit_index])
}
