use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;
use std::thread::sleep;
use std::time::Duration;

const MAX_THREADS: usize = 80;
const BATCH_SIZE: i128 = 1_000_000 - 1;
fn main() {

    let x: Arc<Mutex<i128>> = Arc::new(Mutex::new(0));
    let thread_pool: ThreadPool = ThreadPool::new(MAX_THREADS);

    loop {
        if thread_pool.active_count() < MAX_THREADS {
            println!("threads: {}", thread_pool.active_count());
            
            sleep(Duration::from_millis(1));
            
            let x_value = *x.lock().unwrap();
            let (start, end) = ranger(x_value);
            *x.lock().unwrap() = end;
            let x_arc = Arc::clone(&x);

            thread_pool.execute(move || {
                for i in start..end {
                    let mut y = calc(i);
                    while y != 1 {
                        y = calc(y);
                    }
                }
                drop(x_arc);
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

fn ranger(x: i128) -> (i128, i128) {
    let start = x + 1;
    let end = start + BATCH_SIZE;
    println!("{} - {}\n", start, end);
    (start, end)
}
