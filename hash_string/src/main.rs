use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

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


    //Hashmap
    let mut scores:HashMap<i32, i32> = HashMap::new();

    scores.insert(1, 10);
    scores.insert(2, 20);
    
    println!("{:?}", scores);


    let team_name = String::from("Blue");
    let team_score = 10;

    let mut team = HashMap::new();
    team.insert(team_name, team_score);

    println!("{:?}", team);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{}", score);

    scores.insert(String::from("Blue"), 100);

    println!("{:?}", scores);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);







    


}
