#![allow(unused_variables)]
#![allow(unused_mut)]
fn main() {

    // Declare a mutable implicit integer
    let mut implicit = 42;

    // Declare a mutable explicit integer
    let mut explicit: u8 = 255;

    // Declare an unsigned explicit integer
    let unsigned: u8 = 42;

    // Declare a signed explicit negative integer
    let signed: i8 = -10;

    let mut float: f32 = 1.62; // default f64

    let mut boolean: bool = true;

    let character: char = '🦀';


    let mut tuple1: (u8, f32, char) = (42, 1.62, character);
    tuple1.1 = 3.14;
    println!("Hello, {}!", tuple1.2);

    let (x, y, z) = tuple1; // destructuring
    
    let mut tuple2 = (tuple1, 255);
    println!("The answer to the ultimate question of life, the universe, and everything is {}.", (tuple2.0).0);

    (tuple2.0).0 = 0x2A;
    println!("The binary of hex 2A is {:b}.", (tuple2.0).0);

    /*
    // Immutable string literal

    let str1 = " Hello, World! "; // &str type

    let str2 = str1.trim(); // owned String type

    let str3 = &str2; // borrow operator

    // Contiguous character array


    let str4 = i.to_string(); // String type (new memory allocation)
*/
} 