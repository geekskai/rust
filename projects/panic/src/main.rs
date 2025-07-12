// use std::fs;
// use std::io;

use std::fs::File;

// use std::io::{self, Read};

// use std::io::ErrorKind;

fn main() {
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // println!("The third element is: {}", v[99]); // This will cause a panic
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(file) => file,
    //             Err(e) => panic!("Failed to create file: {}", e),
    //         },
    //         _ => panic!("An unexpected error occurred: {}", error),
    //     },
    // };

    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|e| panic!("Failed to create file: {}", e))
    //     } else {
    //         panic!("An unexpected error occurred: {}", error)
    //     }
    // });

    // let greeting_file = File::open("hello.txt").unwrap();
    // let greeting_file =
    //     File::open("hello.txt").expect("hello.txt should be included in this project");

    // let username_file_result = File::open("hello.txt");

    // let mut username_file = match username_file_result{
    //     Ok(file) => file,
    //     Err(error) => Err(error),
    // }

    // let mut username = String::new();
    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(error) => Err(error),
    // };

    // let greeting_file = File::open("hello.txt")?;
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     // let mut username = String::new();
//     // File::open("hello.txt")?.read_to_string(&mut username)?;
//     // Ok(username)

//     fs::read_to_string("hello.txt")
// }

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
