use std::env;
use std::fs;
use std::process;

fn main() {

    // The function std::env::args returns an iterator of the command line arguments that were passed to minigrep:
    let args: Vec<String> = env::args().collect();
    // Iterators produce a series of values, and we can call the collect method on an iterator to turn it into a collection, such as a vector, containing all the elements the iterator produces.
    
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {}", err);
        process::exit(1);
    }); // Destructures the variables returned by parse_config.
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // fs::read_to_string takes the filename, opens that file, and returns a Result<String> of the file’s contents:
    let contents = fs::read_to_string(config.filename)
        .expect("Error reading the file.");

    println!("With text:\n{}", contents);
}

// A struct to name the related purpose of query and filename and to be able to return the values’ names as struct field names from the Config::new().
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        let query = args[1].clone(); // The program’s name takes up the first value in the vector at args[0], so we start at [1].
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}