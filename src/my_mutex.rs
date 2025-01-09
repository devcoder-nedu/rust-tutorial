use std::{ops::AddAssign, sync::Mutex};

use std::thread::{spawn, scope};

pub fn test_mutex() {
    let score = Mutex::new(0u16);

    // let unlocked_data = score.lock();

    // let mut data = unlocked_data.unwrap();
    // data.add_assign(5);

    // println!("{:?}", data);

    let my_func = || {
        println!("thread 1 is waiting for mutex lock ");
        let mut data = score.lock().unwrap();
        for i in 1..=10 {
            data.add_assign(i);
            println!("thread 1 is adding {i}");
        }
    };

    let my_func2 = || {
        println!("thread 2 is waiting for mutex lock ");
        let mut data = score.lock().unwrap();
        for i in 1..=10 {
            data.add_assign(i);
            println!("thread2 is adding {i}");
        }
    };

    _ = scope(|scope| {
        let handle2 = scope.spawn(my_func2).join();
        let handle1 = scope.spawn(my_func).join();

        if handle1.is_err() {
            println!("let's handle the error that thread 1 had ");
        };
    });



    println!("{}", score.lock().unwrap());
}