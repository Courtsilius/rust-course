use std::io;
use std::str::FromStr;

use crate::bmi::Bmi;
use crate::height::Height;
use crate::weight::Weight;
use crate::error::BmiError;
use crate::bmi_functions::bmi_mod::calculate_bmi;

mod weight;
mod height;
mod bmi;
mod error;
mod bmi_functions;
mod tests;

// BMI calculator
fn main() {
    println!("Bitte Gewicht eingeben (in kg): ");
    let weight: Weight = Weight(get_f64_from_input());

    println!("Bitte Größe eingeben (in cm): ");
    let height: Height = Height(get_f64_from_input() / 100.0);

    // kg / m^2 = BMI
    let bmi = calculate_bmi(height, weight);
    match bmi {
        Ok(bmi) => println!("Dein BMI: {}", bmi.value()),
        Err(e) => println!("<insert insult here>"),
    }
}

fn get_f64_from_input() -> f64 {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_n) => {}
        Err(error) => panic!("error: {error}"),
    };
    f64::from_str(buffer.trim()).unwrap()
}