use std::fs;


pub fn read_file() {
    let file_to_read = "./data/file01.txt";
    let read_result = fs::read(file_to_read);
    if read_result.is_ok() {
        let process_bytes_to_string = |mut a:String, v:&u8| {
            let newchar = char::from(*v);
            a.push(newchar);
            return a;
        };
        println!("Data found is {}", read_result.ok().unwrap() .iter().fold(String::from(""), process_bytes_to_string));
    }
    else {
        println!("Error is {}", read_result.err().unwrap());
    }

}


pub fn create_files() { 
    println!("Creating files");
    let path01 = "./data/file01.txt";
    let path02 = "./data/file02.txt";
    let path03 = "./data/file03.txt";
    let text1 = "Chinedu Mgbemena writes";
    let text2 = "Chinedu Mgbemena learns ";
    let text3 = "Chinedu Mgbemena reads";

    _ = std::fs::write(path01, text1);
    _ = std::fs::write(path02, text2);
    _ = std::fs::write(path03, text3);



    // _ = fs::remove_file(path03);
}


pub fn remove_dir() {
    _ = std::fs::remove_dir_all("./data");
    
}







pub fn test_create_dir () {
    let path = "./data"; 
    let my_path = std::path::Path::new(path);
    if my_path.exists() {
        println!("directory already exists... Ending operation.... ");
        return;
    }
    let create_dir_result =  fs::create_dir(path);
    if create_dir_result.is_ok() {
        println!("Created new directory");
    }
    else {
        println!("Failed to create directory : {:?}",create_dir_result.err());
    }
}