use std::fs::read_to_string;


fn find_first_a(s: &String) -> Option<i64> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i64);
        }
    }
    None
}
fn find (s: &str) -> Option<i64> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i64);
        }
    }
    None
}

fn read_file(file_name: &str) -> Result<String, std::io::Error> {
    let op = read_to_string(&file_name);
    op
}

fn main() {
    let my_string = String::from("hello world");
    match find_first_a(&my_string){
        Some(index) => println!("Found 'a' at index: {}", index),
        None => println!("No 'a' found"),
    }

    println!("{}",my_string);

    let my_string = "hell a world";

    match find(my_string) {
        Some(index) => println!("Found 'a' at index: {}", index),
        None => println!("No 'a' found"),
    }

    println!("{}",my_string);

    let  f = read_file("v.txt");

    match f {
        Ok(contents) => println!("{}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}
