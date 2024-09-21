fn even_filter (vec: &Vec<i32>) -> Vec<i32> {
    let mut even_vec = Vec::new();
    for i in 0..vec.len() {
        if vec[i] % 2 == 0 {
            even_vec.push(vec[i]);
        }
    }
    even_vec
}

fn odd_remover(vec: &mut Vec<i32>) {
    let mut i = 0;
    while i < vec.len() {
        if vec[i] % 2 != 0 {
            vec.remove(i);
        } else {
            i += 1;
        }
    }
}

fn main() {
    let mut vec: Vec<i32> = Vec::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);


    for i in 0..vec.len() {
        if vec[i] % 2 == 0 {
            println!("{} is even", vec[i]);
        } else {
            println!("{} is odd", vec[i]);
        }
    }

    let even_vec = even_filter(&vec);
    for i in 0..even_vec.len() {
        println!("{} is even", even_vec[i]);
    }


    odd_remover(&mut vec);
    

    dbg!(vec);

    let vansh: Vec<i128> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("{:?}", vansh);


}
