use std::io;
use std::str::FromStr;

struct Weight(f64);

struct Height(f64);

struct Bmi(f64);

#[derive(Debug)]
enum BmiError {
    HeightCannotBeZeroOrSmaller,
}

// BMI calculator
fn main() {
    println!("Bitte Gewicht eingeben (in kg): ");
    let weight: Weight = Weight(get_f64_from_input());

    println!("Bitte Größe eingeben (in cm): ");
    let height: Height = Height(get_f64_from_input() / 100.0);

    // kg / m^2 = BMI
    let bmi = bmi(height, weight);
    match bmi {
        Ok(Bmi(val)) => println!("Dein BMI: {}", val),
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

// calculates bmi based on height and weight
fn bmi(height: Height, weight: Weight) -> Result<Bmi, BmiError> {
    if height.0 <= 0.0 {
        return Err(BmiError::HeightCannotBeZeroOrSmaller);
    }

    let bmi = weight.0 / (f64::powf(height.0, 2.0));
    Ok(Bmi(bmi))
}

#[test]
fn test_bmi() {
    let result = bmi(Height(1.69), Weight(69.0)).unwrap();
    assert_eq!(result.0, 24.158817968558527)
}

#[test]
fn test_bmi_fail() {
    let opt = bmi(Height(0.0), Weight(1.69));
    assert!(opt.is_err());
}
