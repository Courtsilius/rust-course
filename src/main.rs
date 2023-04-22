use std::f32;
use std::str::FromStr;
use std::io;
use std::io::Write;
use std::io::Stdin;

// BMI calculator
fn main() {
    let mut stdin = std::io::stdin();

    println!("Bitte Gewicht eingeben (in kg): ");
    let mut weight:f64 = get_f64_from_input(&mut stdin);

    println!("Bitte Größe eingeben (in cm): ");
    let mut height = get_f64_from_input(&mut stdin) / 100.0;

    // kg / m^2 = BMI
    let bmi = bmi(height, weight);

    println!("Dein BMI: {}",bmi);
}

fn get_f64_from_input(stdin:&Stdin) -> f64 {
    let mut buffer_height = String::new();
    stdin.read_line(&mut buffer_height);
    f64::from_str(buffer_height.trim()).unwrap()
}

// calculates bmi based on height and weight
fn bmi(height:f64, weight:f64) -> f64 {
    weight / (f64::powf(height, 2.0))
}
