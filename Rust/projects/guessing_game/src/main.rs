use rand::Rng; // The Rng trait defines methods that random number generators implement.
use std::cmp::Ordering; // The Ordering type is an enum with the variants Less, Greater, and Equal. 
use std::io;

fn main() {
    // println! is a macro that prints a string to the screen:
    println!("Guess the number!");
    println!("Please input your guess.");


    // String::new() is a function that returns a new instance of a String:
    let mut guess = String::new();
    // String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
    // The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is implemented on a type, in this case String, rather than on a particular instance of a String. Some languages call this a static method.
    // This new function creates a new, empty string. You’ll find a new function on many types, because it’s a common name for a function that makes a new value of some kind.


    // The rand::thread_rng function provides a random number generator - one that is local to the current thread of execution and seeded by the operating system:
    let secret_number = rand::thread_rng().gen_range(1..101);
    // The gen_range method is defined by the Rng trait that was brought into scope with the use rand::Rng statement.
    // The gen_range method takes a range expression as an argument and generates a random number in the range.
    

    // The stdin function returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal:
    io::stdin()
        .read_line(&mut guess)
        // The job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents), so it takes that string as an argument.
        // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
        .expect("Failed to read line.");
        // read_line() also returns a value — in this case, an io::Result.
        // The Result types are enumerations, often referred to as enums. An enumeration is a type that can have a fixed set of values, and those values are called the enum’s variants.
        // For Result, the variants are Ok or Err. The Err variant means the operation failed, and Err contains information about how or why the operation failed.


    // Rust allows us to shadow the previous value of guess with a new one. This feature is often used in situations in which you want to convert a value from one type to another type:
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess for example.
    // The trim method on a String instance will eliminate any whitespace at the beginning and end, including \r\n.
    // The parse method on strings parses a string into some kind of number, specified above as u32.

    println!("You guessed: {}", guess);
    

    // A match expression is made up of arms. An arm consists of a pattern and the code that should be run if the value given to the beginning of the match expression fits that arm’s pattern.
    match guess.cmp(&secret_number) { // The cmp method compares two values and can be called on anything that can be compared.
        Ordering::Less => println!("Too low!"),
        Ordering::Greater => println!("Too high!"),
        Ordering::Equal => println!("You guessed it!"),
    }
}