use std::fs::File;
use std::io::{self,ErrorKind, Read};

pub fn open_file() {
    let file_result = File::open("./hello.txt");
    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("create hello.txt failed, error={:?}", error),
            },
            other_error => {
                panic!("create hello.txt failed, error={:?}", other_error)
            },
        },
    };
}

pub fn unwarp_test() {
    let file_result = File::open("./hello2.txt").unwrap();
}

pub fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("./hello.txt");
    
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username+"hello"),
        Err(error) => Err(error),
    }
}