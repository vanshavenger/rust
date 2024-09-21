fn first_word(s: &String) -> String {
    let mut first_word = String::new();
    for (_, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            break;
        }
        first_word.push(item as char);
    }
    first_word
}

fn find_word_first(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}


fn main() {

    let word = String::from("Vansh Chopraa");

    let first_word = first_word(&word);

    println!("First word is: {}", first_word);



    let word2 = &word[0..5];

    // word.clear(); // This will throw an error because word2 is borrowing word
    println!("First word is: {}", word2);


    let word3 = find_word_first(&word);

    println!("First word is: {}", word3);

    // word.clear(); // This will not throw an error because word3 is borrowing word

    println!("First word is: {}", word3); // This will throw an error because word3 is borrowing word
    println!("First word is: {}", word);
}
