use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() { 
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.";
        assert_eq!(vec!["safe, fast proudctive."], search(query, contents));
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
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
