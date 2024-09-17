#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}



impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn perimeter (&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn debug() {
        println!("Debugging");
    }
    
}

#[derive(Debug)]
enum Direction {
    Up = 100,
    Down,
    Left,
    Right,
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match *self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(length, breadth) => length * breadth,
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
    
}

fn calculate_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(length, breadth) => length * breadth,
        Shape::Triangle(a, b, c) => {
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}

fn main() {
    let user1 = User {
        email: String::from("vanshchopra101@gmia.l.com"),
        username: String::from("vanshavenger"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from(""),
        username: String::from(""),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.active);
    println!("{}", user2.active);

    println!("{}", user1.email);
    println!("{}", user2.email);

    println!("{}", user1.username);
    println!("{}", user2.username);

    println!("{}", user1.sign_in_count);
    println!("{}", user2.sign_in_count);

    println!("{:?}", user1);

   dbg!(&user1);

    println!("{:?}", user1);

    let rect1 = Rect {
        width: 30,
        height: 50,
    };


    println!("{}", rect1.area());
    println!("{}", rect1.perimeter());

    println!("{:?}", rect1);

    dbg!(Rect::debug());


    //enum
    let up = Direction::Up;
    let down = Direction::Down;
    let left = Direction::Left;
    let right = Direction::Right;

    println!("{:?}", up);
    println!("{:?}", down);
    println!("{:?}", left);
    println!("{:?}", right);

    //enum with values
    let circle = Shape::Circle(3.14);
    let rectangle = Shape::Rectangle(3.0, 4.0);
    let triangle = Shape::Triangle(3.0, 4.0, 5.0);

    println!("{}", circle.area());
    println!("{}", rectangle.area());
    println!("{}", triangle.area());


    println!("{}", calculate_area(&circle));
    println!("{}", calculate_area(&rectangle));
    println!("{}", calculate_area(&triangle));



}
