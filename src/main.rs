use std::env;
use std::process;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("{} is the query, {} is the filename.", config.query, config.filename);
}

pub struct Config {
    query: String,
    filename: String
}

impl Config {
    // constructor function
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided in the input");
        }
        let query: String = args[1].clone(); // bad cuz time & space tradeoffs
        let filename: String = args[2].clone(); // bad cuz time & space tradeoffs
        Ok(Config {query, filename})
    }
}
