use std::error::Error;
use std::env;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() { 
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query: &str = "rUsT";
        let contents: &str ="\
Rust:
safe, fast, productive.
Pick three. 
Trust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents)
    );
    }
}

pub fn search_case_insensitive<'a>(
    query: &str, 
    contents: &'a str
) ->Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut output = Vec::new();
        for line in contents.lines() {
        if line.contains(query) {
            output.push(line);
        }
    }
    output
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool
}

impl Config {
    // constructor function
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided in the input");
        }
        let query: String = args[1].clone(); // bad cuz time & space tradeoffs
        let filename: String = args[2].clone(); // bad cuz time & space tradeoffs
        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();
        Ok(
            Config {
                query, 
                filename,
                ignore_case
            }
        )
    }
}
