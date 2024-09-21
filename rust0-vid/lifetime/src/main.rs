//'a is a lifetime annotation, which is a way to tell the Rust compiler
// that the references in the function signature and the return value have the same lifetime.

fn longest_string<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}

struct User<'a> {
    name : &'a str
}

fn main() {
    let longest_str;
    let first_str = String::from("first");
    {
        let second_str = String::from("second");

        longest_str = longest_string(&first_str, &second_str);
        println!("The longest string is: {}", longest_str);
    }
    // println!("The longest string is: {}", longest_str); // Error: second_str does not live long enough

    let first_name = String::from("first"); 
    let user = User {
        name: &first_name
    };

    println!("User name: {}", user.name);

}
