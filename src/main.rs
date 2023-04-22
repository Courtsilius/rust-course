use std::f32;
use std::io;
use std::io::Stdin;
use std::io::Write;
use std::str::FromStr;

struct Weight {
    value: f64,
}

struct Height {
    value: f64,
}

struct BMI {
    value: f64,
}

// BMI calculator
fn main() {
    let mut stdin = std::io::stdin();

    println!("Bitte Gewicht eingeben (in kg): ");
    let weight: Weight = Weight {
        value: get_f64_from_input(&mut stdin),
    };

    println!("Bitte Größe eingeben (in cm): ");
    let height: Height = Height {
        value: get_f64_from_input(&mut stdin) / 100.0,
    };

    // kg / m^2 = BMI
    let bmi = bmi(height, weight);

    println!("Dein BMI: {}", bmi.value);
}

fn get_f64_from_input(stdin: &Stdin) -> f64 {
    let mut buffer_height = String::new();
    stdin.read_line(&mut buffer_height);
    f64::from_str(buffer_height.trim()).unwrap()
}

// calculates bmi based on height and weight
fn bmi(height: Height, weight: Weight) -> BMI {
    let bmi = weight.value / (f64::powf(height.value, 2.0));
    BMI { value: bmi }
}
