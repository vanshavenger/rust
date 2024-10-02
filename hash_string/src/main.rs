use unicode_segmentation::UnicodeSegmentation;

fn main() {

    // Hello in hinde
    let mut hello = String::from("नमस्ते");

    println!("{}", match hello.chars().nth(3) {
        Some(c) => c.to_string(),
        None => "No character at index 3!".to_string(),
    });

    println!("{}", hello);

    hello.push_str(" दुनिया");

    println!("{}", hello);

    hello.push('!');


    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", s);



    // Hello in japanese
    let hello = String::from("こんにちは");

    println!("{}", hello);

    let g = hello.graphemes(true).collect::<Vec<&str>>();

    println!("{:?}", g);


    let s = "The quick (\"brown\") fox can't jump 32.3 feet, right?";
    let w = s.unicode_words().collect::<Vec<&str>>();
    let b: &[_] = &["The", "quick", "brown", "fox", "can't", "jump", "32.3", "feet", "right"];
    if w == b {
        println!("Success");
    } else {
        println!("Failure");
    }



    


}
