use std::thread::spawn;

pub fn test_threads() {
    let mut x = 0u128;
    for i in 0..=500 {
        x+=i;
    }
    println!("\x1b[38;2;100;100;255mMain thread finished a little bit of work... Go check on the worker threads");
}

pub fn spawn_threads() {
    let thread_fn = || {
        let mut x = 0u128;
        for i in 0..=50_000_000 {
            x+=i;
        }
        println!("{x}")
    };


    // test_threads();


    println!("Starting new worker thread");
    let handle = spawn(thread_fn);
    let handle2 = spawn(thread_fn);
    println!("Worker thread completed");

    loop {
        test_threads();
        if handle.is_finished() && handle2.is_finished() {
            println!("All the workers are done, let us get out of here\x1b[0m");
            break;
        }
    }
}