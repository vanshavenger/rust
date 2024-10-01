enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);

    println!("{:?}", vec);

    let v = vec![1, 2, 3, 4, 5];

    println!("{:?}", v);

    println!("{}", v[2]);

    // let vc = &v[5];

    // println!("{}", vc);

    let vc = v.get(5);

    match vc {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }

    let op = vec![1, 2, 3, 4, 5];

    let first = &op[0];

    // op.push(6);



    println!("{}", first);

    for i in &op {
        println!("{}", i);
    }


    let mut vansh = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("Vansh")),
    ];

    vansh.push (SpreadsheetCell::Int(4));

    for i in &vansh {
        match i {
            SpreadsheetCell::Int(value) => println!("{}", value),
            SpreadsheetCell::Float(value) => println!("{}", value),
            SpreadsheetCell::Text(value) => println!("{}", value),
        }
    }

}
