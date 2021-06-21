use std::env;
use std::process;

use minigrep::Config;

fn main() {

    // The function std::env::args returns an iterator of the command line arguments that were passed to minigrep:
    let args: Vec<String> = env::args().collect();
    // Iterators produce a series of values, and we can call the collect method on an iterator to turn it into a collection, such as a vector, containing all the elements the iterator produces.
    
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error parsing arguments: {}", err);
        process::exit(1);
    }); // Destructures the variables returned by parse_config.
    
    println!("Searching for {}", config.query);
    println!("in file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
    // We use if let rather than unwrap_or_else to check whether run returns an Err value and call process::exit(1) if it does. The run function doesn’t return a value that we want to unwrap in the same way that Config::new returns the Config instance. Because run returns () in the success case, we only care about detecting an error, so we don’t need unwrap_or_else to return the unwrapped value because it would only be ().
}