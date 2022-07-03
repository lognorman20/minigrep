use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    Ok(())

}

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    // constructor function
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided in the input");
        }
        let query: String = args[1].clone(); // bad cuz time & space tradeoffs
        let filename: String = args[2].clone(); // bad cuz time & space tradeoffs
        Ok(Config {query, filename})
    }
}
