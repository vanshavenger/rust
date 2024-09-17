use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("Generated Number {}", secret_number);
    loop {
        println!("Plesse input your guess: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to get the input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
        }
    }
    
}
