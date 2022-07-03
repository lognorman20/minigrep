use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    
    let query: &String = &args[1];
    let filename: &String = &args[2];

    let contents = fs::read_to_string(filename)
        .expect("Could not read file");
    
    println!("The poem reads as such:\n{}", contents.trim());
}
