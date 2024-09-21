use std::vec;

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5];
    // mutable reference
    // 1 Return a mutable reference to the next value
    // 2 Return None when the iterator is exhausted
    // 3 The iterator is consumed
    let it = nums.iter_mut();

    for v in it {
        *v += 1;
    }  

    // normal iterator
    // 1 Return a reference to the next value
    // 2 Return None when the iterator is exhausted
    // 3 The iterator is not consumed

    for v in nums.iter() {
        println!("{}", v);
    }

    println!("-----------------");


    let nums = vec![1, 2, 3, 4, 5];
    let mut it2 = nums.iter();


    // Next iterator
    // 1 Return the next value
    // 2 Return None when the iterator is exhausted
    // 3 The iterator is not consumed

    let op = it2.next();
    match op {
        Some(v) => println!("{}", v),
        None => println!("None"),
    }

    println!("{:?}", it2);

    while let Some(v) = it2.next() {
        println!("{}", v);
    }

    // into_iter
    // 1 Take ownership of the collection
    // 2 Return an iterator that takes ownership of the collection
    // 3 The collection is consumed


    println!("-----------------");

    let nums = vec![1, 20, 30, 40, 5];
    let it3 = nums.into_iter();

    for v in it3 {
        println!("{}", v);
    }

    // simple for vc in vec - uses into_iter
    let nums1 = vec![1, 20, 30, 40, 5];
    for v in nums1 {
        println!("{}", v);
    }

    // println!("{:?}", nums1); // error: value borrowed here after move

    // Consuming adaptors

    println!("-----------------");
    let op1 = vec![1, 2, 3, 4, 5];

    let vcc  = op1.iter();

    let summ: i32 = vcc.sum();

    println!("{}", summ);

    // println!("{:?}", vcc); // error: value borrowed here after move

    // iterator adaptors
    // 1 Return a new iterator
    // 2 The original iterator is consumed
    // 3 The new iterator is lazy


    println!("-----------------");

    let op2 = vec![1, 2, 3, 4, 5];

    let vcc2  = op2.iter();

    let vcc3 = vcc2.map(|x| x * 2);

    for v in vcc3 {
        println!("{}", v);
    }



    // println!("{:?}", vcc2); // error: value borrowed here after move

    println!("-----------------");

    let op3 = vec![1, 2, 3, 4, 5];

    let vcc4  = op3.iter();

    let vcc5 = vcc4.filter(|x| *x % 2 == 0);

    for v in vcc5 {
        println!("{}", v);
    }

    // println!("{:?}", vcc4); // error: value borrowed here after move


    println!("-----------------");
    println!("-----------------");
    println!("-----------------");
    println!("-----------------");

    let arr = vec![1, 2, 3, 4, 5,6];

    let arr_iter = arr.iter();

    let odd_double_filer = arr_iter.filter(|x| *x % 2 == 0).map(|x| x * 2);

    let new_arr: Vec<i32> = odd_double_filer.collect();

    // for v in odd_double_filer{
    //     new_arr.push(v);
    // }

    println!("{:?}", new_arr);


    // Same can be used for hashmap






    
}
