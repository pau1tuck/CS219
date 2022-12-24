#![allow(unused_variables)]
#![allow(unused_mut)]

fn main() {
    /* SCALAR TYPES */
    // Declare an implicit integer variable 'forty_two' with a value of 42
    let forty_two = 42;

    // Declare a mutable explicit 8-bit unsigned integer variable 'unsigned' with a value of 255
    let mut unsigned_integer: u8 = 255;

    // Declare an explicit signed 8-bit integer 'signed' with a value of -10
    let signed_integer: i8 = -10;

    // Declare a mutable 32-bit floating-point number variable 'phi' with a value of 1.62
    let mut phi: f32 = 1.62; // default f64

    // Declare a mutable boolean type variable 'boolean' with a value of true
    let mut boolean: bool = true;

    // Declare an explicit character type variable 'rust' with a value of 🦀
    let rust: char = '🦀';


    /* COMPOUND TYPES */
    // Declare an explicit mutable tuple variable 'tuple1' containing variables forty_two, phi, and crab
    let mut tuple1: (u8, f32, char) = (forty_two, phi, rust);

    // Update the floating-point number in tuple1 to 3.14
    tuple1.1 = 3.14;
    // Print "Hello, x!", substituting x with the character variable in tuple1
    println!("Hello, {}!", tuple1.2);

    // Destructure tuple1 into new variables 'x', 'y', and 'z'
    let (x, y, z) = tuple1;

    // Declare an implicit mutable 2-dimensional tuple variable 'tuple2' containing tuple1 and unsigned_integer
    let mut tuple2 = (tuple1, 255); // ((42, 3.14, '🦀'), 255)
    println!("The answer to the ultimate question of life, the universe, and everything is {}.", (tuple2.0).0);

    // Update the value of forty_two in tuple2 to the hexadecimal value 2A
    (tuple2.0).0 = 0x2A;

    // Print "The binary equivalent of hex 2A is x", replacing x with the binary form of the new value of forty_two in tuple2
    println!("The binary equivalent of hex 2A is {:b}.", (tuple2.0).0);

    // Declare the following variable 'days'
    let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];

    println!("I was born on a {}.", days[5]);

    let array1: [i32; 3] = [255, 42, 13];
    println!("The first value in array1 is {}.", array1[0]);


    let str1 = " Hello, World! "; // &str type

    let str2 = str1.trim(); // owned String type

    let str3 = &str2; // borrow operator

    // Contiguous character array


    let str4 = i.to_string(); // String type (new memory allocation)
*/
}