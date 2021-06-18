use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let moon_weight = calculate_weight_on_moon(100);
    println!("Weight on the Moon: {} kg", calculate_weight_on_moon(moon_weight));
}

fn calculate_weight_on_moon(weight: f32) -> f32 {
    (weight / 9.81) * 1.622
}

