pub fn test_vec_int() {
    let mut my_ints:Vec<i32> = Vec::new();
    my_ints.push(5);
    my_ints.push(5);
    my_ints.push(5);
    my_ints.push(5);

    println!("{:?}", my_ints);
    println!("Size of vector {}", my_ints.len());
    println!("Capacity of vector {}", my_ints.capacity());
    println!("First item in the Vec is {:?}", 
    match my_ints.get(6) {
        Some(num) => num,
        None => return,
    }
    ); 
}

pub fn test_vec_string() {
    let first_names = vec!["John", "Josh", "Drake", "Soma", "Nath"];

    for first_name in first_names.clone() {
        println!("{}.....", first_name);
    }
    println!("{}", first_names[0]);
}    

#[derive(Debug)]
struct Car {
    manufacturer : String,
    model : String,
}

pub fn test_vec_car() {
    let mut car_list = Vec::new();
    let mut car_lot = vec![];

    for _ in 1..=100u8 {
        car_list.push(Car{
            manufacturer: "Hyundai".to_string(),
            model: "Elantra".to_string(),
        });

        car_lot.push(Car{
        manufacturer: "Porshe".to_string(),
        model: "Jaguar".to_string(),
        });
    }

    car_list.append(&mut car_lot);
    let keep = |e:&Car| {
        if e.manufacturer == "Porshe".to_string() {
            return true;
        }
        else {
            return false;
        }
    };

    car_list.retain(keep);


    println!("{:?}", car_list);
    println!("{:?}", car_list.len());
    println!("{:?}", car_lot);

    let mut input = String::new();

    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You entered: {}", input.trim());
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
        }
    }
}