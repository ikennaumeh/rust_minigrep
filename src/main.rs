use std::env;
use std::fs;

fn main() {
    let args : Vec<String> = env::args().collect();
    
    let query = &args[1];
    let file_path = &args[2];

    print!("Search for {}", query);
    print!("\nIn file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read file");

    println!("\nWith text: \n{contents}",);
}
