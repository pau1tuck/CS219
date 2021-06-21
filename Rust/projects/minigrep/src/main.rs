use std::env;
use std::fs;

fn main() {

    // The function std::env::args returns an iterator of the command line arguments that were passed to minigrep:
    let args: Vec<String> = env::args().collect();
    // Iterators produce a series of values, and we can call the collect method on an iterator to turn it into a collection, such as a vector, containing all the elements the iterator produces.
    
    
    let config = parse_config(&args); // Destructures the variables returned by parse_config.
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // fs::read_to_string takes the filename, opens that file, and returns a Result<String> of the file’s contents:
    let contents = fs::read_to_string(config.filename)
        .expect("Error reading the file.");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone(); // The program’s name takes up the first value in the vector at args[0], so we start at [1].
    let filename = args[2].clone();

    Config { query, filename }
}