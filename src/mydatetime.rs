extern crate chrono;

use std::time::{Duration, Instant};

use chrono::NaiveDate;

pub fn test_stdtime() {
    let dur1 = Duration::from_secs(9);

    println!("{:?}", dur1.as_micros());

    let dur2 = Duration::from_millis(5500);

    let dur3 = match dur1.checked_sub(dur2) {
        Some(result) => result,
        None => {
            println!("Returning a null value as the operation cant be carried out");
            return;
        },
    };

    println!("{}", dur3.as_millis());

}

pub fn test_chrono() {
    let utc_now = chrono::Utc::now();
    println!("{}", utc_now.format("%Z %Y %b %d"));

    let local_time = chrono::Local::now();
    println!("{}", local_time.format("%Z %Y %b %d"));
    
    let date1 = NaiveDate::from_isoywd_opt(2024, 32, chrono::Weekday::Fri);
    let unwrapped_date = date1.unwrap();
    println!("Day of the year is {}", unwrapped_date.format("%j"));

    let birthday = NaiveDate::parse_from_str("2024,12,5", "%Y,%m,%d");
    println!("{:?}", birthday.unwrap());
}


