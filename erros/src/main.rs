use std::{fs::File, io::Read};

fn divide (x: i32, y: i32) -> Result<f32,String> {
    if y == 0 {
        return Err("Cannot divide by zero".to_string());
    }

    Ok(x as f32 / y as f32)
}

fn main() {
    // Recoverable errors
    let result = std::fs::read_to_string("hello.txt");

    match result {
        Ok(content) => println!("File content: {}", content),
        Err(error) => println!("Error: {}", error),
    }

    // let content = match result {
    //     Ok(content) => content,
    //     Err(error) => {
    //         println!("Error: {}", error);
    //         return;
    //     }
    // };

    // println!("File content: {}", content);


    let op = std::fs::read_to_string("t.txt");

    match op {
        Ok(content) => println!("File content: {}", content),
        Err(error) => println!("Error: {}", error),
        
    }

    // Unrecoverable errors

    // panic!("crash and burn");


    // let v = vec![1, 2, 3];

    // v[99];

    // let x: i32 = 0;

    // let y: i32 = 10;

    // let result = y / x;

    // println!("Result: {}", result);

    let result = divide(10, 4);

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    let greeting_file_request = File::open("hello.txt");

    let mut greeting_file = match greeting_file_request {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    match File::create("hello.txt") {
                        Ok(fc) => fc22,
                        Err(e) => {
                            panic!("Tried to create file but there was a problem: {:?}", e)
                        }
                    }
                }
                other_error => {
                    panic!("There was a problem opening the file: {:?}", other_error)
                }
                
            }
        }
    };

    let mut content = String::new();

    match &greeting_file.read_to_string(&mut content) {
        Ok(_) => println!("File content: {}", content),
        Err(error) => println!("Error: {}", error),
    }


    
}
