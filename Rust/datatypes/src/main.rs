#![allow(unused_variables)]
#![allow(unused_mut)]

fn main() {
    /* SCALAR TYPES */
    // Declare an implicit integer with a value of 42
    let forty_two = 42;

    // Declare a mutable explicit 8-bit unsigned integer with a value of 255
    let mut explicit: u8 = 255;

    // Declare an explicit signed 8-bit integer with a value of -10
    let signed: i8 = -10;

    // Declare a mutable 32-bit floating-point number with a value of 1.62
    let mut phi: f32 = 1.62; // default f64

    // Declare a mutable boolean type with a value of true
    let mut boolean: bool = true;

    // Declare a character type with a value of 🦀
    let crab: char = '🦀';


    /* COMPOUND TYPES */
    let mut tuple1: (u8, f32, char) = (forty_two, phi, crab);
    tuple1.1 = 3.14;
    println!("Hello, {}!", tuple1.2);

    let (x, y, z) = tuple1; // destructuring
    
    let mut tuple2 = (tuple1, 255); // ((42, 3.14, '🦀'), 255)
    println!("The answer to the ultimate question of life, the universe, and everything is {}.", (tuple2.0).0);

    (tuple2.0).0 = 0x2A;
    println!("The binary equivalent of hex 2A is {:b}.", (tuple2.0).0);

    let days = ["Monday", "Tuesday", "Wednesday", "hursday", "Friday", "Saturday", "Sunday"];

    let array1: [i32; 3] = [255, 42, 13];
    println!("The first value in array1 is {}.", array1[0]);

    println!("I was born on a {}.", days[5]);

    let empty_string = String::new();

    let my_string1 = String::from("Hello, Rust!");

    /*
    // Immutable string literal

    
    let str1 = " Hello, World! "; // &str type

    let str2 = str1.trim(); // owned String type

    let str3 = &str2; // borrow operator

    // Contiguous character array


    let str4 = i.to_string(); // String type (new memory allocation)
*/
} 