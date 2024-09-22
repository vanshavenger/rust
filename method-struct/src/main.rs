#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    fn say_hello(&self) {
        println!("Hello, my name is {} and I am {} years old", self.name, self.age);
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
     self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

fn main() {
    let person = Person {
        name: String::from("Vansh"),
        age: 20,
    };

    println!("{:?}", person);

    let person = Person::new(String::from("Vansh"), 20);
    person.say_hello();



    println!("------------------------------------");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("The area of the rectangle is {} square pixels", rect1.area());


    // if rec1 acan handle rect2 then print Y
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    // if rec2 acan handle rect1 then print Y
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));

    println!("------------------------------------");

    let sq = Rectangle::square(3);
    println!("The area of the square is {} square pixels", sq.area());

    println!("------------------------------------");
    

    let home = IpAddrKind::V4(String::from("17.99.990"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);

    println!("------------------------------------");

    let config_max = Some(3_u8);

    match config_max {
        Some(i) => println!("Max value is {}", i),
        None => (),
    }

    if let Some(i) = config_max {
        println!("Max value is {}", i);
    }


}
