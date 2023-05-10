use inquire::CustomType;


use crate::bmi::Bmi;
use crate::bmi_functions::bmi_mod::calculate_bmi;
use crate::error::BmiError;
use crate::height::Height;
use crate::weight::Weight;
use crate::file_handler::file_handler_mod::save;

mod bmi;
mod bmi_functions;
mod error;
mod height;
mod tests;
mod weight;
mod file_handler;

// BMI calculator
fn main() {
    env_logger::init();

    let weight_input = CustomType::<f64>::new("Bitte Gewicht eingeben (in kg): ")
        .with_formatter(&|i| format!("{:.2}kg", i))
        .with_error_message("Please type a valid number")
        .with_help_message("1 Gewicht bitte")
        .prompt();

    match weight_input {
        Ok(weight_input) => log::debug!("User input weight: {}", weight_input),
        Err(_) => panic!("I cant keep doing this."),
    }

    let height_input = CustomType::<f64>::new("Bitte Größe eingeben (in cm): ")
        .with_formatter(&|i| format!("{}cm", i))
        .with_error_message("Please type a valid number")
        .with_help_message("1 Groß bitte in cm.")
        .prompt();

    match height_input {
        Ok(height_input) => log::debug!("User input height: {}", height_input),
        Err(_) => panic!("I cant keep doing this."),
    }

    let weight = Weight(weight_input.unwrap());
    let height = Height(height_input.unwrap() / 100.0);

    // kg / m^2 = BMI
    let bmi = calculate_bmi(height, weight);
    match bmi {
        Ok(ref bmi) => println!("Dein BMI: {}", bmi.value()),
        Err(_e) => panic!("Get rekt"),
    }

    save(bmi.expect("REASON"));
}
