use std::error::Error;
use std::fs;

// A struct to name the related purpose of query and filename and to be able to return the values’ names as struct field names from the Config::new().
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        let query = args[1].clone(); // The program’s name takes up the first value in the vector at args[0], so we start at [1].
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{ // This function previously returned the unit type
    // Box<dyn Error> means the function will return a type that implements the dynamic Error trait,
    // fs::read_to_string takes the filename, opens that file, and returns a Result<String> of the file’s contents:
    let contents = fs::read_to_string(config.filename)?;
    // Rather than panic! on an error, ? will return the error value from the current function for the caller to handle.

    println!("With text:\n{}", contents);
        Ok(())
    // We’ve declared the run function’s success type as () in the signature, which means we need to wrap the unit type value in the Ok value. This Ok(()) syntax might look a bit strange at first, but using () like this is the idiomatic way to indicate that we’re calling run for its side effects only; it doesn’t return a value we need.
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}