use std::sync::{Arc, Mutex};
use std::time::Duration;
use threadpool::ThreadPool;

const MAX_THREADS: usize = 16;
const BATCH_SIZE: i128 = (1_000_000_000_000 - 1) / MAX_THREADS as i128;

fn main() {
    let checker: Arc<Mutex<i128>> = Arc::new(Mutex::new(0));
    let x: Arc<Mutex<i128>> = Arc::new(Mutex::new(0));
    let thread_pool: ThreadPool = ThreadPool::new(MAX_THREADS);

    loop {
        std::thread::sleep(Duration::from_millis(100));

        if thread_pool.active_count() <= MAX_THREADS - 1
            && (*x.lock().unwrap() / BATCH_SIZE) - *checker.lock().unwrap() <= MAX_THREADS as i128
        {
            println!(
                "threads: {} checkers: {}",
                thread_pool.active_count(),
                *checker.lock().unwrap()
            );

            let x_value = *x.lock().unwrap();
            let (start, end) = ranger(x_value);
            *x.lock().unwrap() = end;
            let x_arc = Arc::clone(&x);
            let checker_arc = Arc::clone(&checker);

            thread_pool.execute(move || {
                for i in start..end {
                    if i % 100_000 == 0 {
                        let max: i128 = start - end;
                        let done: i128 = start - i;
                        let percentage: f64 = (done as f64 / max as f64 ) * 100.0;
                        println!("{}%",percentage);                        
                    }
                    calc(i);
                }
                let mut checker = checker_arc.lock().unwrap();
                *checker += 1;
                drop(x_arc);
            });
        }
    }
}

fn calc(x: i128) {
    // its messy because i wanted to add caching but gave up
    let mut y = c(x);
    while y != 1 {
        y = c(y);
    }
}
fn c(b: i128) -> i128 {
    if b % 2 == 0 {
        b / 2
    } else {
        3 * b + 1
    }
}

fn ranger(x: i128) -> (i128, i128) {
    let start = x + 1;
    let end = start + BATCH_SIZE;
    println!("{} - {}\n", start, end);
    (start, end)
}
