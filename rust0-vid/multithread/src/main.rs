use std::{sync::mpsc, thread};

fn main() {
    // let handles: Vec<_> = (0..3).map(|thread_id| {
    //     thread::spawn(move || {
    //         for i in 1..5_i64 {
    //             let data = vec![i; 10000]; 
                
    //             let sum: i64 = data.iter().sum();
    //             println!("Hi number {} from thread {}! Sum: {}", i, thread_id, sum);
    //         }
    //     })
    // }).collect();

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // for i in 1..1_00_i64 {
    //     let data = vec![i; 10000];
    //     let sum: i64 = data.iter().sum();
    //     println!("Hi number {} from the main thread! Sum: {}", i, sum);
    // }

    // let (tx,rx) = mpsc::channel();


    // thread::spawn(move || {
    //     for i in 1..5 {
    //         let data = vec![i; 10000]; 
    //         let sum: i64 = data.iter().sum();
    //         println!("Hi number {} from the thread! Sum: {}", i, sum);
    //         tx.send(sum).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    let (tx,rx ) = mpsc::channel();

    for i in 0..10 {
        let producer = tx.clone();
        thread::spawn(move || {
            let mut ans = 0;
            for j in 0..100_00_00_0 {
                ans += i*100_00_00_0 + j;
            }
            producer.send(ans).unwrap();
        });
    }
    drop(tx);

    let mut ans: u64 = 0;
    for rec in rx {
        ans += rec;
        println!("Received: {}", rec);
    }

    println!("Final sum: {}", ans);


    

}
