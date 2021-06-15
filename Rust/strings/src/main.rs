fn main() {

    let mut string1 = String::new();
    let string2 = String::from("Hello");
    let string3 = "World".to_string();
    println!("{}, {}!", string2, string3);

    string1.push_str("Rust");
    string1.push('🦀');
    println!("{}, {}!", string2, string1);
}