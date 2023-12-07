use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn _panic_func1() {
    panic!("crash and burn");
}

fn _panic_func2() {
    let v = vec![1, 2, 3];
    v[99];
}

fn file_open() {
    let _f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}

fn file_open1() {
    let _f = File::open("hello.txt").unwrap();
}

fn file_open2() {
    let _f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    file_open();
    file_open1();
    file_open2();
    match read_username_from_file() {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e)
    }
}
