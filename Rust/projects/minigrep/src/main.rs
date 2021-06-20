use std::env;
use std::fs;

fn main() {

    // The function std::env::args returns an iterator of the command line arguments that were passed to minigrep.
    // Iterators produce a series of values, and we can call the collect method on an iterator to turn it into a collection, such as a vector, containing all the elements the iterator produces:
    let args: Vec<String> = env::args().collect();

    let query = &args[1]; // The program’s name takes up the first value in the vector at args[0], so we start at [1].
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    // fs::read_to_string takes the filename, opens that file, and returns a Result<String> of the file’s contents:
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");

    println!("With text:\n{}", contents);
}