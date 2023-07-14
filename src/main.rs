use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;
use std::thread::sleep;
use std::time::Duration;

const MAX_THREADS: usize = 8;
const BATCH_SIZE: i128 = 1_000_000;
fn main() {

    let x: Arc<Mutex<i128>> = Arc::new(Mutex::new(0));
    let thread_pool: ThreadPool = ThreadPool::new(MAX_THREADS);

    loop {
        if thread_pool.active_count() < MAX_THREADS {
            println!("threads: {}", thread_pool.active_count());
            
            sleep(Duration::from_millis(1));
            
            let (range, x2) = ranger(*x.lock().unwrap());
            *x.lock().unwrap() = x2;
            thread_pool.execute(move || {
                for i in range {
                    let mut y = calc(i);
                    while y != 1 {
                        y = calc(y);
                    }
                }
            });
        }
    }
}

fn calc(x: i128) -> i128 {
    if x % 2 == 0 {
        x / 2
    } else {
        3 * x + 1
    }
}

fn ranger(x3: i128) -> (std::ops::Range<i128>, i128) {
    let end = x3 + BATCH_SIZE;
    println!("{}", end);
    (x3..end, end)
}
