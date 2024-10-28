// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.


use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // 使用 join 方法等待线程完成并获取返回值，方法返回一个 Result<T, E>，其中 T 是目标线程的返回值类型，E 是可能的错误类型。
        // expect("Thread panicked") 用于处理 Result，如果目标线程发生panic，程序会打印 "Thread panicked" 并终止。
        // 如果返回OK()，则会拆包出真正的数据。
        let result = handle.join().expect("Thread panicked");
        results.push(result);
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
