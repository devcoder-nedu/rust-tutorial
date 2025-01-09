pub fn test_iters () {
    let fruit_list = vec!["Strawberry", "Blueberry", "Mango", "Orange", "Apple"];
    let nut_list = vec!["Walnut", "Almond", "Pecans", "Brazil Nuts"];


    let mut fruit_iter = fruit_list.iter();

    let item01 = fruit_iter.next();

    println!("{}", item01.unwrap());

    let iter = fruit_list.iter().chain(&nut_list);
    println!("{:?}", iter);

    for food in iter {
        println!("{}", food);
    };


    let fruit_list_strings = fruit_list.iter().map(|e| String::from(*e));
    let updated_fruits_list = fruit_list_strings.map(|mut e| {e.push_str(" fruit"); return e});

    updated_fruits_list.clone().for_each(|e| println!("{:?}", e));

    println!("Last one is   {:?}", updated_fruits_list.clone().last().unwrap());

}