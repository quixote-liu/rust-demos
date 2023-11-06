use std::fmt::Display;


#[derive(Debug)]
enum FileState {
    Open,
    Closed,
}

impl Display for FileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FileState::Open => write!(f, "file open"),
            FileState::Closed => write!(f, "file close"),
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    path: String,
    state: FileState,
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name = {}, path = {}, state = {}", self.name, self.path, self.state)
    }
}

impl File {
    fn new(name: String, path: String, state: FileState) -> Self {
        File { name: name, path: path, state: state }
    }
}

pub fn display_demo_run() {
    let file = File::new("filename".to_string(), "filepath".to_string(), FileState::Open);
    println!("FILE = {:?}", file);
}
