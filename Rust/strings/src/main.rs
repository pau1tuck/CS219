fn main() {

    // String literals
    let greeting = String::from("Hello");
    let person = "Ferris".to_string();
    println!("{}, {}!", greeting, person);

    
    let mut my_string = String::new();
    my_string.push_str(&greeting);
    my_string.push_str(", ");
    my_string.push('🦀');
    my_string.push('!');

    println!("{}", my_string);

    let owned = my_string.trim(); // owned String type

    let borrowed = &owned; // borrowed &str type
}