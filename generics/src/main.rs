struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn new (x: T, y: U) -> Self {
        Point {
            x,
            y,
        }
    }

    fn mixup<V, W> (self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let numbers = vec![1, 2, 3, 40, 5, 6, 7, 8, 9, 10];

    let result = largest_value(&numbers);

    println!("The largest value is {}", result);

    println!("{:?}", numbers);

    let numbers2 = vec![1.0, 2.0, 3.0, 40.0, 55.5334, 6.0, 7.0, 8.0, 9.0, 10.0];

    let result2 = largest_value2(&numbers2);

    println!("The largest value is {}", result2);


    let numbers3 = vec![1, 2, 3, 40, 5, 6, 7, 8, 9, 10];

    let result3 = largest_generic(&numbers3);

    println!("The largest value is {}", result3);

    
    let numbers4 = vec![1.0, 2.0, 3.0, 40.0, 55.5334, 6.0, 7.0, 8.0, 9.0, 10.0];

    let result4 = largest_generic(&numbers4);

    println!("The largest value is {}", result4);

    let char_list = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];

    let result5 = largest_generic(&char_list);

    println!("The largest value is {}", result5);

    println!("{:?}", char_list);

    let string_list = vec!["hello", "world", "rust", "programming", "language"];

    let result6 = largest_generic(&string_list);

    println!("The largest value is {}", result6);

    println!("{:?}", string_list);

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x);

    let p2 = Point { x: 5.0, y: 10.5 };

    println!("p2.y = {}", p2.y);

    println!("p2.x = {}", p2.x());

    println!("p2.y = {}", p2.y());

    let p3 = Point::new(5, 10);

    println!("p3.x = {}", p3.x);

    let p4 = Point::new(5.44, 10.5);

    println!("p4.y = {}", p4.y);

    println!("p4.x = {}", p4.x());

    println!("p4.y = {}", p4.y());

    let p5 = Point::new(5, 10);

    let p6 = Point::new(5.44, 10.5);

    let p7 = p5.mixup(p6);

    println!("p7.x = {}", p7.x);

    println!("p7.y = {}", p7.y);


}

fn largest_value (list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_value2 (list: &[f32]) -> &f32 {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_generic<T: PartialOrd> (list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}