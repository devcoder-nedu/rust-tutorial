struct Person {
    firstname: String,
}

pub fn test_threads_variables() {
    let age = 34u16;
    let person01 = Person{firstname: "Chinedu".to_string()};

    let print_age = ||  {
        println!("the age of the person is {age}");
        println!("the person's firstname is {}", &person01.firstname);
    };

    // let _result  = std::thread::spawn(print_age).join();

    std::thread::scope(|scope| {
        scope.spawn(print_age);
    });

    println!("the age of the person is {age}");
    println!("the person's firstname is {}", person01.firstname);

    println!("Finished printing age");
}