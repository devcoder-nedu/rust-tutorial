use::std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

pub fn test_channels() {
    let (tx , rx):(Sender<u8>, Receiver<u8>) = mpsc::channel();
    
    let processor_code = move || {
        println!("Starting Processor Thread... ");
        loop {
            println!("Attempting to receive message from the channel");
            let receiver_result = rx.recv_timeout(Duration::from_millis(800));
            if receiver_result.is_ok() {
                println!("received result is {}", receiver_result.unwrap());
            }
            
        }

    };
    for x in 1..=6 {
        let send_result = tx.send(x);

        println!("{}", send_result.is_ok());
    }

    std::thread::spawn(processor_code).join();


}