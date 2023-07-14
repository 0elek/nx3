use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Instant, Duration};
use threadpool::ThreadPool;

fn main() {
    let x = Arc::new(Mutex::new(1));
    let thread_pool = ThreadPool::new(8);
    let start = Instant::now();

    for _ in 0..8 {
        let (range, x2) = ranger(*x.lock().unwrap());
        *x.lock().unwrap() = x2;

        for i in range {
            let x = Arc::clone(&x);
            thread_pool.execute(move || {
                let mut y = calc(i);
                while y != 1 {
                    y = calc(y);
                }
            });
        }
    }

    thread_pool.join();
    println!("Time taken: {:?}", start.elapsed());
}

fn calc(x: i128) -> i128 {
    if x % 2 == 0 {
        x / 2
    } else {
        3 * x + 1
    }
}

fn ranger(x3: i128) -> (std::ops::Range<i128>, i128) {
    let end = x3 + 100000;
    (x3..end, end)
}
