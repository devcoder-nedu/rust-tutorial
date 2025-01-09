use std::env::args;

pub fn get_user_input() {
    let my_args:Vec<String> = args().collect();
    println!("{:?}", my_args);

    if my_args.len() != 3 {
        println!("You did not specify two arguments");
        return;
    }

    let name:String = my_args.get(1).unwrap().into();
    let year_born = my_args.get(2).unwrap().parse::<i32>().ok().unwrap();


    println!("{} {}",name, year_born);

    let dog01 = new_dog (name,  year_born);
    dog01.get_dog_details();
}

fn new_dog(name:String, year_born:i32) -> Dog {
    return Dog{name, year_born};
}

#[derive(Debug)]
struct Dog {
    name: String,
    year_born: i32,
}

impl Dog {
    fn get_dog_details(&self) {
        println!("Dog name is {}, dog year of birth is {}", self.name, self.year_born);
    }
}