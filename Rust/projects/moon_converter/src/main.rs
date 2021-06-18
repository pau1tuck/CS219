use std::io;

fn main() {
    println!("Enter your weight (kg):");
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    let earth_weight: f32 = input.trim().parse().unwrap();

    let moon_weight = calculate_weight_on_moon(earth_weight);
    println!("Your weight on the Moon is {:.2} kg.", moon_weight);
}

fn calculate_weight_on_moon(weight: f32) -> f32 {
    // Weight on Earth / 9.81 m/s^2) * 1.622 m/s^2
    (weight / 9.81) * 1.622
}