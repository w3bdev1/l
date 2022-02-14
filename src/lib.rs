use std::path::PathBuf;

// Colors
const RESET_COLOR: &str = "\x1b[0m";
const BLUE_BOLD: &str = "\x1b[94;1m";

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
                let name = String::from(item_path.file_name().unwrap().to_str().unwrap());
                let is_dir = item_path.is_dir();

                files.push(File { name, is_dir })
            }
        }

        Files::new(files)
    }

    pub fn render(self) {
        for f in self.files {
            match f.is_dir {
                true => {
                    println!("• {}{}{}", &BLUE_BOLD, f.name, RESET_COLOR)
                }
                false => {
                    println!("• {}", f.name)
                }
            }
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    is_dir: bool,
}
