use std::collections::HashMap;


fn group_values_by_key(vec: Vec<(String,i32)>) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for (key, value) in vec {
        map.insert(key, value);
    }
    map
}
fn main() {
    // hash map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    let vans = scores.get("Blue");

    match vans {
        Some(v) => println!("Blue: {}", v),
        None => println!("No value found"),
    }

    let test = scores.get("Red");

    match test {
        Some(v) => println!("Red: {}", v),
        None => println!("No value found"),
    }

    
    let mp = group_values_by_key(vec![
        ("A".to_string(), 1),
        ("B".to_string(), 2),
        ("A".to_string(), 3),
        ("B".to_string(), 4),
    ]);


    println!("{:?}", mp);



    

}
