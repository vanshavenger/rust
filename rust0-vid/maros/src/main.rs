fn main() {

    // macros - println!
    println!("Hello, world!");
    // println!("Hello, world!");
    // macro explanation - println! is a macro that prints text to the console and appends a newline.
    // println! is a macro, not a function. The ! indicates that println! is a macro.

    // variables
    let x = 5;
    println!("The value of x is: {}", x);
    

    let vc = vec![1, 2, 3, 4, 5];

    for i in &vc {
        println!("{}", i);
    }

    println!("Hello, world!");
}
